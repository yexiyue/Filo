# Stronghold + Zustand Persist 集成

## 概述

使用 Tauri Stronghold 插件作为 Zustand persist 的存储后端，实现安全的状态持久化。

## 实现

### 1. StrongholdStorage 类

```typescript
// src/lib/stronghold.ts
import { StateStorage } from "zustand/middleware";
import { Client, Stronghold } from "@tauri-apps/plugin-stronghold";
import { appDataDir } from "@tauri-apps/api/path";

// 实现 zustand StateStorage 接口的 Stronghold 存储适配器
class StrongholdStorage implements StateStorage {
  // Stronghold 的 key-value 存储实例
  private store: Awaited<ReturnType<Client["getStore"]>>;
  // Stronghold 实例，用于保存数据到磁盘
  private stronghold: Stronghold;

  // 私有构造函数，强制使用 create 工厂方法
  private constructor(
    store: Awaited<ReturnType<Client["getStore"]>>,
    stronghold: Stronghold
  ) {
    this.store = store;
    this.stronghold = stronghold;
  }

  // 工厂方法：异步初始化 Stronghold 并返回存储实例
  static async create(password: string, clientName = "filo") {
    // vault 文件存储在应用数据目录
    const vaultPath = `${await appDataDir()}/vault.hold`;
    // 使用密码加载或创建 vault
    const stronghold = await Stronghold.load(vaultPath, password);
    let client: Client;
    try {
      // 尝试加载已存在的 client
      client = await stronghold.loadClient(clientName);
    } catch {
      // 不存在则创建新的 client
      client = await stronghold.createClient(clientName);
    }
    return new StrongholdStorage(client.getStore(), stronghold);
  }

  // StateStorage 接口：获取数据
  getItem = async (name: string) => {
    const data = await this.store.get(name);
    if (!data) return null;
    // Stronghold 存储的是字节数组，需要解码为字符串
    return new TextDecoder().decode(new Uint8Array(data));
  };

  // StateStorage 接口：存储数据
  setItem = async (name: string, value: string) => {
    // 字符串编码为字节数组
    const data = Array.from(new TextEncoder().encode(value));
    await this.store.insert(name, data);
    // 每次写入后持久化到磁盘
    await this.stronghold.save();
  };

  // StateStorage 接口：删除数据
  removeItem = async (name: string) => {
    await this.store.remove(name);
    await this.stronghold.save();
  };
}

// 顶层 await 初始化，确保模块导入时 Stronghold 已就绪
export const strongholdStorage = await StrongholdStorage.create("filo-stronghold");
```

### 2. Zustand Store

```typescript
// src/hooks/use-secret-store.ts
import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import { strongholdStorage } from "@/lib/stronghold";

// Store 状态类型定义
interface SecretState {
  keypair: number[] | null;      // Ed25519 密钥对字节数组
  deviceId: string | null;       // 从公钥派生的设备 ID
  _hasHydrated: boolean;         // 标记 hydration 是否完成
  setHasHydrated: (state: boolean) => void;
  init: () => Promise<void>;     // 初始化方法：检查并创建密钥对
}

export const useSecretStore = create(
  persist<SecretState>(
    (set, get) => ({
      keypair: null,
      deviceId: null,
      _hasHydrated: false,
      setHasHydrated: (state) => set({ _hasHydrated: state }),
      // 初始化：如果没有密钥对则生成新的
      async init() {
        const { keypair } = get();
        if (!keypair) {
          const keypair = await generateKeypair();
          const deviceId = await registerKeypair(keypair);
          set({ keypair, deviceId });
        }
      },
    }),
    {
      name: "secret-store",  // 存储的 key 名称
      // 必须用 createJSONStorage 包装，处理 JSON 序列化
      storage: createJSONStorage(() => strongholdStorage),
    }
  )
);

// 关键：异步存储必须使用 onFinishHydration 回调
// 在 hydration 完成后执行初始化逻辑
useSecretStore.persist.onFinishHydration((state) => {
  state.init();              // 检查/创建密钥对
  state.setHasHydrated(true); // 标记 hydration 完成
});
```

## 注意事项

### 1. 异步存储的 Hydration 问题

Zustand persist 默认假设存储是同步的。使用异步存储（如 Stronghold）时：

- **不能**依赖自动 hydration
- **必须**使用 `onFinishHydration` 回调处理初始化逻辑
- 使用 `_hasHydrated` 状态追踪 hydration 是否完成

### 2. 组件中的使用

```tsx
function Component() {
  const { data, _hasHydrated } = useSecretStore();

  // 必须等待 hydration 完成
  if (!_hasHydrated) {
    return <Loading />;
  }

  return <div>{data}</div>;
}
```

### 3. createJSONStorage 包装

必须使用 `createJSONStorage(() => strongholdStorage)` 包装自定义存储，否则 JSON 序列化/反序列化不会正确工作。

### 4. 顶层 await

`strongholdStorage` 使用顶层 await 初始化，确保在使用前 Stronghold 已加载完成。需要 ES2022+ 或相应的构建配置支持。

### 5. 数据持久化

每次 `setItem` 后调用 `stronghold.save()` 确保数据写入磁盘。如果频繁写入，考虑添加 debounce。

## TypeScript 工具类型

### `Awaited<T>`

`Awaited<T>` 是 TypeScript 内置的工具类型，用于提取 Promise 的解析值类型：

```typescript
type A = Awaited<Promise<string>>;           // string
type B = Awaited<Promise<Promise<number>>>;  // number (递归解包)
type C = Awaited<string>;                    // string (非 Promise 直接返回)
```

在代码中的应用：

```typescript
private store: Awaited<ReturnType<Client["getStore"]>>;
```

- `Client["getStore"]` - 获取 `getStore` 方法的类型
- `ReturnType<...>` - 获取该方法的返回类型
- `Awaited<...>` - 解包 Promise，得到实际的 `Store` 类型

好处：不需要手动查找 `Store` 类型的导出名，直接从方法签名推导。

## 参考

- [Tauri Stronghold Plugin](https://v2.tauri.app/plugin/stronghold/)
- [Zustand Persist Middleware](https://docs.pmnd.rs/zustand/integrations/persisting-store-data)
- [异步存储处理](https://juejin.cn/post/7553861170454560820)
