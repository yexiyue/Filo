use anyhow::anyhow;
use async_trait::async_trait;
use libp2p::{swarm::SwarmEvent, PeerId};

use crate::network::commands::{CommandHandler, SharedStatHandle};

#[derive(Debug)]
pub struct CloseCommand {
    peer_id: PeerId,
}

impl CloseCommand {
    pub fn new(peer_id: PeerId) -> Self {
        Self { peer_id }
    }
}

#[async_trait]
impl CommandHandler for CloseCommand {
    type Result = ();

    async fn run(
        &mut self,
        swarm: &mut crate::network::FiloSwarm,
        handler: &SharedStatHandle<Self::Result>,
    ) {
        if swarm.disconnect_peer_id(self.peer_id.clone()).is_err() {
            let err = anyhow!("no active connection with {}", self.peer_id);
            handler.finish(Err(err.into())).await;
        }
    }

    async fn on_swarm_event(
        &mut self,
        event: &SwarmEvent<crate::network::FiloBehaviourEvent>,
        handler: &SharedStatHandle<Self::Result>,
    ) -> bool {
        match event {
            SwarmEvent::ConnectionClosed { peer_id, .. } if *peer_id == self.peer_id => {
                handler.finish(Ok(())).await;
                false
            }
            _ => true,
        }
    }
}
