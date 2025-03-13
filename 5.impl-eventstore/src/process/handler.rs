use crate::markers::{Aggregate, Command, Event};
use crate::process::Context;
use async_trait::async_trait;

#[async_trait]
pub trait CommandHandler<C: Command>: Aggregate {
    type Event: Event;
    type Rejection: 'static + Sync + Send;

    async fn handle(&self, command: C, ctx: &mut Context) -> Result<Self::Event, Self::Rejection>;
}

#[async_trait]
pub trait EventApplicator<E: Event>: Aggregate {
    async fn apply(&mut self, event: E, ctx: &mut Context);
}
