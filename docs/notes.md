# Filo 开发注意事项

开发过程中遇到的问题和解决方案记录。

---

## 1. GossipSub `add_explicit_peer` vs `swarm.dial`

### 问题

在处理 mDNS 发现节点时，应该使用 `add_explicit_peer` 还是 `swarm.dial`？

### 区别

| 方法 | 作用 | 是否建立连接 |
|------|------|-------------|
| `swarm.dial(addr)` | 在 TCP 层建立物理连接 | ✅ 立即建立 |
| `gossipsub.add_explicit_peer(&peer_id)` | 将节点标记为显式对等节点，优先保持在 Mesh 中 | ❌ 不直接建立，但 GossipSub 会自动处理 |

### 官方推荐做法

根据 [rust-libp2p 官方示例](https://github.com/libp2p/rust-libp2p)，**只需要 `add_explicit_peer`，不需要手动 `dial`**：

```rust
// 官方示例
SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(peers))) => {
    for (peer_id, _) in peers {
        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
        // 不需要 dial
    }
}
```

### 原因

1. **mDNS 是双向广播** - 当 A 发现 B 时，B 也可能同时发现 A 并尝试连接
2. **GossipSub 自动 dial** - 订阅同一 topic 时，GossipSub 会自动连接 explicit peers 建立 Mesh
3. **避免重复连接** - 双方都 dial 会建立两条连接，浪费资源

### 结论

- **使用 GossipSub 时**：只需 `add_explicit_peer`，不需要手动 `dial`
- **不使用 GossipSub 或需要立即建立连接时**：使用 `dial`

---

## 2. 建立连接后发送数据的几种方式

### 方式一：GossipSub（发布订阅）

适合广播消息给所有订阅者：

```rust
// 订阅 topic
let topic = gossipsub::IdentTopic::new("filo/sync");
swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

// 发布消息
swarm.behaviour_mut().gossipsub.publish(topic, data)?;

// 接收消息
FiloBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. }) => {
    let data = message.data;
}
```

### 方式二：request-response（点对点请求）

适合向特定节点发送请求并等待响应，需要在 `FiloBehaviour` 中添加：

```rust
pub request_response: request_response::cbor::Behaviour<Request, Response>,
```

使用：

```rust
// 发送请求
swarm.behaviour_mut().request_response.send_request(&peer_id, request);

// 处理响应
RequestResponseEvent::Message {
    message: RequestResponseMessage::Response { response, .. }, ..
}
```

### 方式三：Stream（底层流）

libp2p 0.54+ 支持直接打开流进行读写：

```rust
let stream = swarm.behaviour_mut()
    .new_control()
    .open_stream(peer_id, "/filo/stream/1.0.0")
    .await?;
// 用 stream 读写数据
```

### 选择建议

| 场景 | 推荐方式 |
|------|----------|
| 元数据同步/设备发现 | GossipSub |
| 文件传输/需要响应 | request-response |
| 大文件流式传输 | Stream |

---

## 3. （待补充）

