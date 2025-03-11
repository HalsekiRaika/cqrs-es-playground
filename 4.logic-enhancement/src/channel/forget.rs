use std::fmt::Debug;
use async_trait::async_trait;
use crate::channel::ProcessApplier;
use crate::errors::ProcessDropped;
use crate::handler::{CommandHandler, EventApplicator};
use crate::markers::{Aggregate, Command};

pub struct NonblockingReceptor<C: Command> {
    pub(crate) command: C
}

#[async_trait]
impl<C: Command, T: Aggregate> ProcessApplier<T> for NonblockingReceptor<C> 
where 
    T: CommandHandler<C> 
    + EventApplicator<T::Event>,
    T::Rejection: Debug
{
    async fn apply(self: Box<Self>, entity: &mut T) -> Result<(), ProcessDropped> {
        match entity.handle(self.command).await {
            Ok(ev) => {
                entity.apply(ev).await
            }
            Err(e) => {
                tracing::error!("{:?}", e);
            }
        }
        Ok(())
    }
}