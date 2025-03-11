mod command;
mod event;
mod forget;

use async_trait::async_trait;
use std::fmt::Debug;
use tokio::sync::mpsc::UnboundedSender;

use crate::channel::command::CommandReceptor;
use crate::channel::event::EventReceptor;
use crate::channel::forget::NonblockingReceptor;
use crate::errors::ProcessDropped;
use crate::handler::{CommandHandler, EventApplicator};
use crate::markers::{Aggregate, Command, Event};

#[async_trait]
pub trait ProcessApplier<T: Aggregate>: 'static + Sync + Send {
    async fn apply(self: Box<Self>, entity: &mut T) -> Result<(), ProcessDropped>;
}

pub struct Receptor<T: Aggregate> {
    channel: UnboundedSender<Box<dyn ProcessApplier<T>>>,
}

impl<T: Aggregate> Receptor<T> {
    pub fn new(channel: UnboundedSender<Box<dyn ProcessApplier<T>>>) -> Self {
        Self { channel }
    }
}

impl<T: Aggregate> Receptor<T> {
    pub async fn handle<C: Command>(&self, command: C) -> Result<T::Event, T::Rejection>
    where
        T: CommandHandler<C>,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.channel
            .send(Box::new(CommandReceptor {
                command,
                oneshot: tx,
            }))
            .unwrap();

        rx.await.unwrap()
    }

    pub async fn apply<E: Event>(&self, event: E)
    where
        T: EventApplicator<E>,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.channel
            .send(Box::new(EventReceptor { event, oneshot: tx }))
            .unwrap();

        rx.await.unwrap()
    }

    pub async fn entrust<C: Command>(&self, command: C)
    where
        T: CommandHandler<C> + EventApplicator<T::Event>,
        T::Rejection: Debug,
    {
        self.channel
            .send(Box::new(NonblockingReceptor { command }))
            .unwrap();
    }
}
