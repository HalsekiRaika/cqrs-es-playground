use crate::channel::ProcessApplier;
use crate::errors::ChannelDropped;
use crate::handler::CommandHandler;
use crate::markers::{Aggregate, Command};
use async_trait::async_trait;
use tokio::sync::oneshot;

pub struct CommandReceptor<C: Command, T: Aggregate>
where
    T: CommandHandler<C>,
{
    pub(crate) command: C,
    pub(crate) oneshot: oneshot::Sender<Result<T::Event, T::Rejection>>,
}

#[async_trait]
impl<C: Command, T: Aggregate> ProcessApplier<T> for CommandReceptor<C, T>
where
    T: CommandHandler<C>,
{
    async fn apply(self: Box<Self>, entity: &mut T) -> Result<(), ChannelDropped> {
        self.oneshot
            .send(entity.handle(self.command).await)
            .map_err(|_| ChannelDropped)
    }
}
