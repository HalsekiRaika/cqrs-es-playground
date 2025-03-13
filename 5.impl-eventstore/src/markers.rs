use crate::errors::{DeserializeError, SerializeError};
use crate::identifier::EntityId;

// 集約を表すトレイト
pub trait Aggregate: 'static + Sync + Send + Sized {
    fn id(&self) -> EntityId;
}

// コマンドを表すマーカートレイト
pub trait Command: 'static + Sync + Send + Sized {}

// イベントを表すトレイト
pub trait Event: 'static + Sync + Send + Sized {
    const REGISTRY_KEY: &'static str;
    
    fn to_bytes(&self) -> Result<Vec<u8>, SerializeError>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, DeserializeError>;
}
