use crate::Result;
use libp2p::identity::Keypair;
use tokio::sync::mpsc;

pub mod client;
pub use client::*;
pub mod swarm;
pub use swarm::*;
pub mod event_loop;
pub use event_loop::*;
pub mod commands;
pub use commands::NetCmd;

pub fn start(keypair: &Keypair) -> Result<NetClient> {
    let (command_sender, command_receiver) = mpsc::channel(32);
    let filo_swarm = FiloSwarm::new(keypair)?;
    let mut event_loop = event_loop::EventLoop::new(filo_swarm, command_receiver);

    let client = NetClient::new(command_sender);

    event_loop.start_listen()?;

    tokio::spawn(event_loop.run());
    Ok(client)
}
