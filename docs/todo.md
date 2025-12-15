# Filo 开发计划

## v0.1 - 基础连接

- [x] 密钥对生成与持久化
- [ ] 创建 Swarm (TCP + Noise + Yamux)
- [ ] mDNS 节点发现
- [ ] 验证：两台设备能互相发现并建立连接

## v0.2 - 直连与信任

- [ ] PeerConnectInfo 连接信息 (filo:// URL)
- [ ] KnownPeers 已知节点注册表
- [ ] TrustManager 信任管理
- [ ] 手动输入 PeerId 直连
- [ ] 验证：通过 filo:// 链接添加好友并建立信任连接

## v0.3 - 点对点消息

- [ ] FiloMessage 消息协议
- [ ] 点对点消息收发
- [ ] Tauri 命令封装 (p2p_start, peer_connect, send_message)
- [ ] 前端 TypeScript API
- [ ] 验证：两设备能互发消息

## v0.4 - 房间模式

- [ ] GossipSub 集成
- [ ] 房间号生成与 Topic 订阅
- [ ] RoomManager (创建/加入/离开)
- [ ] 验证：多人房间消息广播

## v0.5 - Yjs 协作

- [ ] Yjs 消息类型 (YjsUpdate, YjsSyncRequest, YjsSyncResponse)
- [ ] FiloProvider 实现
- [ ] 新成员加入同步
- [ ] 验证：多设备实时协作编辑

## v0.6 - 文件同步

- [ ] FastCDC 分块 + BLAKE3 哈希
- [ ] FileManifest 清单
- [ ] request-response 协议
- [ ] 差异同步与文件重建
- [ ] 验证：文件变更能增量同步

## v0.7 - 安全增强

- [ ] 可选房间 E2EE (AES-256-GCM)
- [ ] 验证：加密房间消息不可被非成员读取

## 待定

- [ ] Awareness (光标、选区同步)
- [ ] 离线支持与冲突处理
