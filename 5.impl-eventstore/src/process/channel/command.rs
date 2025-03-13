use crate::process::channel::ProcessApplier;
use crate::errors::ChannelDropped;
use crate::process::handler::CommandHandler;
use crate::markers::{Aggregate, Command};
use async_trait::async_trait;
use tokio::sync::oneshot;
use crate::process::Context;

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
    async fn apply(self: Box<Self>, entity: &mut T, ctx: &mut Context) -> Result<(), ChannelDropped> {
        self.oneshot
            .send(entity.handle(self.command, ctx).await)
            .map_err(|_| ChannelDropped)
    }
}
