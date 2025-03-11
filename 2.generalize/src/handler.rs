use crate::markers::{Aggregate, Command, Event};
use async_trait::async_trait;

#[async_trait]
pub trait CommandHandler<C: Command>: Aggregate {
    type Event: Event;
    type Rejection: 'static + Sync + Send;

    async fn handle(&self, command: C) -> Result<Self::Event, Self::Rejection>;
}

pub async fn handle<T, C: Command>(entity: &T, command: C) -> Result<T::Event, T::Rejection>
where
    T: CommandHandler<C>,
{
    entity.handle(command).await
}

#[async_trait]
pub trait EventApplicator<E: Event>: Aggregate {
    async fn apply(&mut self, event: E);
}

pub async fn apply<T, E: Event>(entity: &mut T, event: E)
where
    T: EventApplicator<E>,
{
    entity.apply(event).await;
}
