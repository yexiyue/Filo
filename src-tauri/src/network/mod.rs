use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
    io,
    time::Duration,
};

use anyhow::Result;
use libp2p::{
    futures::StreamExt,
    gossipsub,
    identify::{self, Config},
    identity::Keypair,
    mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, PeerId, Swarm, SwarmBuilder,
};

/// Filo网络行为组合
///
/// 这个结构体组合了Filo应用所需的各种libp2p网络行为:
/// - identify: 用于节点间相互识别和获取节点信息
/// - mdns: 用于本地网络中的节点自动发现
/// - gossipsub: 用于实现Gossip协议的消息传播
#[derive(NetworkBehaviour)]
pub struct FiloBehaviour {
    /// 节点识别行为，用于交换节点信息如版本号、支持的协议等
    pub identify: identify::Behaviour,
    /// mDNS行为，用于在本地网络中自动发现其他Filo节点
    pub mdns: mdns::tokio::Behaviour,
    /// Gossipsub行为，实现基于Gossip协议的消息传播机制
    pub gossipsub: gossipsub::Behaviour,
}

/// Filo网络Swarm管理器
///
/// 负责管理整个libp2p网络栈，包括连接管理、消息处理等
pub struct FiloSwarm {
    /// libp2p Swarm实例，包含所有的网络行为和连接状态
    pub swarm: Swarm<FiloBehaviour>,
    /// 受信任的节点集合，用于访问控制和安全验证
    pub trussed_peers: HashSet<PeerId>,
}

impl FiloSwarm {
    /// 创建新的Filo网络Swarm实例
    ///
    /// # 参数
    /// * `keypair` - 节点的身份密钥对，用于加密和身份验证
    ///
    /// # 返回值
    /// 返回Result包装的FiloSwarm实例，如果创建过程中出现错误则返回错误信息
    pub async fn new(keypair: Keypair) -> Result<Self> {
        let swarm = SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            // 配置TCP传输层，使用Noise进行加密，Yamux作为多路复用器
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            // 配置网络行为组合
            .with_behaviour(|keypair| {
                let peer_id = keypair.public().to_peer_id();

                // 定义Gossip消息ID生成函数，用于去重和消息追踪
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                // 配置Gossipsub行为参数
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    // 设置心跳间隔为10秒，有助于调试且不会产生过多日志
                    .heartbeat_interval(Duration::from_secs(10))
                    // 使用严格验证模式，强制要求消息签名
                    .validation_mode(gossipsub::ValidationMode::Strict)
                    // 使用消息内容作为ID，相同内容的消息不会重复传播
                    .message_id_fn(message_id_fn)
                    .build()
                    .map_err(io::Error::other)?;

                // 创建Gossipsub行为实例
                let gossipsub = gossipsub::Behaviour::new(
                    // 使用签名验证确保消息来源的真实性
                    gossipsub::MessageAuthenticity::Signed(keypair.clone()),
                    gossipsub_config,
                )?;

                // 创建mDNS行为实例，用于本地网络发现
                let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?;

                // 创建Identify行为实例，用于节点间信息交换
                let identify = identify::Behaviour::new(
                    Config::new("/filo/id/1.0.0".to_string(), keypair.public())
                        // 设置代理版本信息，便于识别Filo节点版本
                        .with_agent_version(format!("filo/{}", env!("CARGO_PKG_VERSION"))),
                );

                // 组合所有行为
                Ok(FiloBehaviour {
                    mdns,
                    identify,
                    gossipsub,
                })
            })?
            .build();

        Ok(FiloSwarm {
            swarm,
            trussed_peers: HashSet::new(),
        })
    }
}
