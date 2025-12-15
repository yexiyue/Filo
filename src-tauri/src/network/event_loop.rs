use crate::network::{commands::NetCmd, FiloBehaviourEvent, FiloSwarm};

use libp2p::{futures::StreamExt, swarm::SwarmEvent};
use tokio::sync::mpsc;
use tracing::{info, trace};

pub struct EventLoop {
    swarm: FiloSwarm,
    command_receiver: mpsc::Receiver<NetCmd>,
    active: Vec<NetCmd>,
}

impl EventLoop {
    pub fn new(swarm: FiloSwarm, command_receiver: mpsc::Receiver<NetCmd>) -> Self {
        Self {
            swarm,
            command_receiver,
            active: vec![],
        }
    }

    pub async fn handle_command(&mut self, mut cmd: NetCmd) {
        cmd.run(&mut self.swarm).await;
        self.active.push(cmd);
    }

    pub async fn handle_event(&mut self, event: SwarmEvent<FiloBehaviourEvent>) {
        let mut i = 0;
        while i < self.active.len() {
            if self.active[i].on_swarm_event(&event).await {
                i += 1;
            } else {
                self.active.swap_remove(i);
            }
        }
        trace!("Event: {:#?}", event);
    }

    pub(crate) async fn run(mut self) {
        loop {
            tokio::select! {
                cmd= self.command_receiver.recv() => match cmd{
                    Some(cmd) => self.handle_command(cmd).await,
                    None => return,
                },
                event = self.swarm.select_next_some() => self.handle_event(event).await,
            }
        }
    }

    pub fn start_listen(&mut self) -> anyhow::Result<()> {
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        Ok(())
    }
}
