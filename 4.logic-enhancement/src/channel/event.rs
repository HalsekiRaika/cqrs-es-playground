use async_trait::async_trait;
use tokio::sync::oneshot;
use crate::channel::ProcessApplier;
use crate::errors::ProcessDropped;
use crate::handler::EventApplicator;
use crate::markers::{Aggregate, Event};

pub struct EventReceptor<E: Event> {
    pub(crate) event: E,
    pub(crate) oneshot: oneshot::Sender<()>,
}

#[async_trait]
impl<E: Event, T: Aggregate> ProcessApplier<T> for EventReceptor<E>
where
    T: EventApplicator<E>
{
    async fn apply(self: Box<Self>, entity: &mut T) -> Result<(), ProcessDropped> {
        self.oneshot
            .send(entity.apply(self.event).await)
            .map_err(|_| ProcessDropped)
    }
}
