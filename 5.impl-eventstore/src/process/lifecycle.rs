use crate::process::channel::{ProcessApplier, Receptor};
use crate::markers::Aggregate;
use crate::process::Context;

pub async fn run<T: Aggregate>(entity: T) -> Receptor<T> {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Box<dyn ProcessApplier<T>>>();

    let receptor = Receptor::new(tx);
    let context = Context::new(0);

    tokio::spawn(async move {
        let mut entity = entity;
        let mut context = context;
        
        while let Some(handler) = rx.recv().await {
            if let Err(e) = handler.apply(&mut entity, &mut context).await {
                tracing::error!("{e}");
            }
        }
    });

    receptor
}
