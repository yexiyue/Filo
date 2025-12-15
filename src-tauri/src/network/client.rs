use crate::{
    network::{
        commands::{CloseCommand, CommandFuture, DialCommand},
        NetCmd,
    },
    Result,
};
use libp2p::PeerId;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct NetClient {
    tx: mpsc::Sender<NetCmd>,
}

impl NetClient {
    pub fn new(tx: mpsc::Sender<NetCmd>) -> Self {
        Self { tx }
    }

    pub async fn dial(&self, peer_id: PeerId) -> Result<()> {
        let dial_command = DialCommand::new(peer_id);
        CommandFuture::new(dial_command, self.tx.clone()).await?;
        Ok(())
    }

    pub async fn close(&self, peer_id: PeerId) -> Result<()> {
        let close_command = CloseCommand::new(peer_id);
        CommandFuture::new(close_command, self.tx.clone()).await?;
        Ok(())
    }
}
