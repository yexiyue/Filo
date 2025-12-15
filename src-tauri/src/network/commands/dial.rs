use anyhow::anyhow;
use async_trait::async_trait;
use libp2p::{core::connection::ConnectedPoint, swarm::SwarmEvent, PeerId};

use crate::network::commands::{CommandHandler, SharedStatHandle};

#[derive(Debug)]
pub struct DialCommand {
    peer_id: PeerId,
}

impl DialCommand {
    pub fn new(peer_id: PeerId) -> Self {
        Self { peer_id }
    }
}

#[async_trait]
impl CommandHandler for DialCommand {
    type Result = ();

    async fn run(
        &mut self,
        swarm: &mut crate::network::FiloSwarm,
        handler: &SharedStatHandle<Self::Result>,
    ) {
        if let Err(error) = swarm.dial(self.peer_id.clone()) {
            let err = anyhow!("failed to dial {}: {error}", self.peer_id);
            handler.finish(Err(err.into())).await;
        }
    }

    async fn on_swarm_event(
        &mut self,
        event: &SwarmEvent<crate::network::FiloBehaviourEvent>,
        handler: &SharedStatHandle<Self::Result>,
    ) -> bool {
        match event {
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } if *peer_id == self.peer_id && matches!(endpoint, ConnectedPoint::Dialer { .. }) => {
                handler.finish(Ok(())).await;
                false
            }
            SwarmEvent::OutgoingConnectionError {
                peer_id: Some(peer_id),
                error,
                ..
            } if *peer_id == self.peer_id => {
                let err = anyhow!("failed to dial {}: {error}", self.peer_id);
                handler.finish(Err(err.into())).await;
                false
            }
            _ => true,
        }
    }
}
