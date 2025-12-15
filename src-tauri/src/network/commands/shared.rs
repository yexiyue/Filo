use crate::network::{FiloBehaviourEvent, FiloSwarm};
use crate::Result;
use libp2p::swarm::SwarmEvent;
use std::any::Any;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use tokio::sync::Mutex;
use async_trait::async_trait;

#[async_trait]
pub trait CommandHandler {
    type Result: Any;

    async fn run(&mut self, swarm: &mut FiloSwarm, handler: &SharedStatHandle<Self::Result>);

    async fn on_swarm_event(
        &mut self,
        event: &SwarmEvent<FiloBehaviourEvent>,
        handler: &SharedStatHandle<Self::Result>,
    ) -> bool {
        let _ = event;
        let _ = handler;
        false
    }
}

#[derive(Debug)]
pub struct SharedState<T> {
    pub result: Option<Result<T>>,
    pub waker: Option<Waker>,
}

impl<T> Default for SharedState<T> {
    fn default() -> Self {
        Self {
            result: None,
            waker: None,
        }
    }
}

#[derive(Debug)]
pub struct SharedStatHandle<T>(Arc<Mutex<SharedState<T>>>);

impl<T> Clone for SharedStatHandle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> SharedStatHandle<T> {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }

    pub fn poll(&self, ctx: &Context<'_>) -> Poll<Result<T>> {
        let mut state = self.0.try_lock().unwrap();
        if let Some(result) = state.result.take() {
            Poll::Ready(result)
        } else {
            state.waker.replace(ctx.waker().clone());
            Poll::Pending
        }
    }

    pub async fn finish(&self, result: Result<T>) {
        let mut state = self.0.lock().await;
        state.result.replace(result);

        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}
