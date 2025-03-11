use crate::channel::{ProcessApplier, Receptor};
use crate::markers::Aggregate;

pub async fn run<T: Aggregate>(entity: T) -> Receptor<T> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Box<dyn ProcessApplier<T>>>();

    let receptor = Receptor::new(tx);

    tokio::spawn(async move {
        let mut entity = entity;
        while let Some(handler) = rx.recv().await {
            if let Err(e) = handler.apply(&mut entity).await {
                tracing::error!("{e}");
            }
        }
    });

    receptor
}
