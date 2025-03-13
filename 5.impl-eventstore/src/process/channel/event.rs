use crate::process::channel::ProcessApplier;
use crate::errors::ChannelDropped;
use crate::process::handler::EventApplicator;
use crate::markers::{Aggregate, Event};
use async_trait::async_trait;
use tokio::sync::oneshot;
use crate::process::Context;

pub struct EventReceptor<E: Event> {
    pub(crate) event: E,
    pub(crate) oneshot: oneshot::Sender<()>,
}

#[async_trait]
impl<E: Event, T: Aggregate> ProcessApplier<T> for EventReceptor<E>
where
    T: EventApplicator<E>,
{
    async fn apply(self: Box<Self>, entity: &mut T, ctx: &mut Context) -> Result<(), ChannelDropped> {
        self.oneshot
            .send(entity.apply(self.event, ctx).await)
            .map_err(|_| ChannelDropped)?;
        ctx.seq += 1;
        Ok(())
    }
}
