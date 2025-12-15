use crate::network::{FiloBehaviourEvent, FiloSwarm};
use async_trait::async_trait;
use libp2p::swarm::SwarmEvent;

pub type NetCmd = Box<dyn NetCommand + Send + 'static>;

#[async_trait]
pub trait NetCommand: Send {
    async fn run(&mut self, swarm: &mut FiloSwarm);
    async fn on_swarm_event(&mut self, event: &SwarmEvent<FiloBehaviourEvent>) -> bool;
}
