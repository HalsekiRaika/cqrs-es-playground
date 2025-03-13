use std::collections::BTreeSet;
use async_trait::async_trait;
use crate::errors::{ReadError, WriteError};
use crate::eventstore::payload::SerializedEvent;

#[async_trait]
pub trait Reader: 'static + Sync + Send {
    async fn read_to(&self, aggregate_id: &str, from: i64, to: i64) -> Result<BTreeSet<SerializedEvent>, ReadError>;
    async fn read_to_latest(&self, aggregate_id: &str, from: i64) -> Result<BTreeSet<SerializedEvent>, ReadError> {
        self.read_to(aggregate_id, from, i64::MAX).await
    }
}

#[async_trait]
pub trait Writer: 'static + Sync + Send {
    async fn append(&self, event: SerializedEvent) -> Result<(), WriteError>;
}