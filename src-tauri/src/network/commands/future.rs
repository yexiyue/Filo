use crate::{
    network::{FiloBehaviourEvent, FiloSwarm},
    Result,
};
use anyhow::anyhow;
use async_trait::async_trait;
use libp2p::swarm::SwarmEvent;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::mpsc;

use super::net_command::{NetCmd, NetCommand};
use super::shared::{CommandHandler, SharedStatHandle};

pub struct CommandFuture<T>
where
    T: CommandHandler + Send + 'static,
    T::Result: Send + 'static,
{
    command: Option<T>,
    handler: SharedStatHandle<T::Result>,
    sender: mpsc::Sender<NetCmd>,
}

impl<T> CommandFuture<T>
where
    T: CommandHandler + Send + 'static,
    T::Result: Send + 'static,
{
    pub fn new(cmd: T, sender: mpsc::Sender<NetCmd>) -> Self {
        let handler = SharedStatHandle::new();
        Self {
            command: Some(cmd),
            handler,
            sender,
        }
    }
}

impl<T> Future for CommandFuture<T>
where
    T: CommandHandler + Send + Unpin + 'static,
    T::Result: Send + 'static,
{
    type Output = Result<T::Result>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if let Some(command) = this.command.take() {
            let task = CommandTask::new(command, this.handler.clone());
            match this.sender.try_send(Box::new(task)) {
                Ok(_) => Poll::Pending,
                Err(_) => Poll::Ready(Err(anyhow!("command channel closed").into())),
            }
        } else {
            this.handler.poll(cx)
        }
    }
}

struct CommandTask<T>
where
    T: CommandHandler + Send + 'static,
    T::Result: Send + 'static,
{
    command: T,
    handler: SharedStatHandle<T::Result>,
}

impl<T> CommandTask<T>
where
    T: CommandHandler + Send + 'static,
    T::Result: Send + 'static,
{
    fn new(command: T, handler: SharedStatHandle<T::Result>) -> Self {
        Self { command, handler }
    }
}

#[async_trait]
impl<T> NetCommand for CommandTask<T>
where
    T: CommandHandler + Send + 'static,
    T::Result: Send + 'static,
{
    async fn run(&mut self, swarm: &mut FiloSwarm) {
        self.command.run(swarm, &self.handler).await;
    }

    async fn on_swarm_event(&mut self, event: &SwarmEvent<FiloBehaviourEvent>) -> bool {
        self.command.on_swarm_event(event, &self.handler).await
    }
}
