use crate::handler::{CommandHandler, EventApplicator};
use crate::markers::Command;
use std::fmt::Debug;
use tokio::sync::mpsc::UnboundedSender;

pub async fn run<T, C: Command>(entity: T) -> UnboundedSender<C>
where
    T: CommandHandler<C> + EventApplicator<T::Event>,
    T::Rejection: Debug,
{
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<C>();
    tokio::spawn(async move {
        let mut entity = entity;
        while let Some(command) = rx.recv().await {
            match entity.handle(command).await {
                Ok(ev) => entity.apply(ev).await,
                Err(e) => {
                    tracing::error!("{:?}", e);
                }
            }
        }
    });
    tx
}
