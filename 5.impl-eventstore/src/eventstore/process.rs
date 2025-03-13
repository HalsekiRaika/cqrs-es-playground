use crate::eventstore;
use crate::markers::{Aggregate, Event};
use crate::process::Context;

pub trait WithPersistence: Aggregate {
    async fn persist<E: Event>(&self, event: &E, ctx: &Context) {
        if let Err(e) = eventstore::get_eventstore()
            .append(self.id().as_ref(), ctx.seq(), event)
            .await
        {
            panic!("Failed to persist event: {:?}", e);
        }
    }
}
