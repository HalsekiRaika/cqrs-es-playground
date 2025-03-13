use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(sqlx::FromRow)]
pub struct SerializedEvent {
    pub id: String,
    pub key: String,
    pub seq: i64,
    pub bytes: Vec<u8>,
    pub created_at: time::OffsetDateTime
}

impl Debug for SerializedEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SerializedEvent")
            .field("id", &self.id)
            .field("key", &self.key)
            .field("seq", &self.seq)
            .field("bytes", &format!("<{} bytes>", self.bytes.len()))
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl PartialEq for SerializedEvent {
    fn eq(&self, other: &Self) -> bool {
        (self.id.eq(&other.id) 
            && self.key.eq(&other.key) 
            && self.seq.eq(&other.seq))
            || self.bytes.eq(&other.bytes)
    }
}

impl Eq for SerializedEvent {}

impl PartialOrd for SerializedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SerializedEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seq.cmp(&other.seq)
            .then_with(|| self.created_at.cmp(&other.created_at))
    }
}
