pub mod sqlite;

use crate::errors::{DeserializeError, ReadError, WriteError};
use crate::eventstore::io::{Reader, Writer};
use crate::eventstore::payload::SerializedEvent;
use crate::markers::Event;
use std::collections::BTreeMap;
use std::sync::Arc;

pub trait Adaptor: 'static + Sync + Send
where
    Self: Reader + Writer,
{
}

pub struct EventStore {
    pub(in crate::eventstore) store: Option<Arc<dyn Adaptor>>,
}

impl Clone for EventStore {
    fn clone(&self) -> Self {
        match self.store {
            None => Self { store: None },
            Some(ref store) => Self {
                store: Some(Arc::clone(store)),
            },
        }
    }
}

impl<T> From<T> for EventStore
where
    T: Adaptor,
{
    fn from(store: T) -> Self {
        Self {
            store: Some(Arc::new(store)),
        }
    }
}

impl EventStore {
    pub async fn read_to<E: Event>(
        &self,
        id: &str,
        from: i64,
        to: i64,
    ) -> Result<BTreeMap<i64, E>, ReadError> {
        let Some(store) = &self.store else {
            panic!("EventStore is not installed.");
        };
        
        let read = store
            .read_to(id, from, to)
            .await?
            .into_iter()
            .map(|serialized| Ok((serialized.seq, E::from_bytes(&serialized.bytes)?)))
            .collect::<Result<BTreeMap<i64, E>, DeserializeError>>()?;
        Ok(read)
    }

    pub async fn read_to_latest<E: Event>(
        &self,
        id: &str,
        from: i64,
    ) -> Result<BTreeMap<i64, E>, ReadError> {
        let Some(store) = &self.store else {
            panic!("EventStore is not installed.");
        };
        
        let read = store
            .read_to_latest(id, from)
            .await?
            .into_iter()
            .map(|serialized| Ok((serialized.seq, E::from_bytes(&serialized.bytes)?)))
            .collect::<Result<BTreeMap<i64, E>, DeserializeError>>()?;
        Ok(read)
    }

    pub async fn append<E: Event>(&self, id: &str, seq: i64, event: &E) -> Result<(), WriteError> {
        let Some(store) = &self.store else {
            panic!("EventStore is not installed.");
        };
        
        let serialized = SerializedEvent {
            id: id.to_string(),
            key: E::REGISTRY_KEY.to_string(),
            seq,
            bytes: event.to_bytes()?,
            created_at: time::OffsetDateTime::now_utc(),
        };
        store.append(serialized).await
    }
}
