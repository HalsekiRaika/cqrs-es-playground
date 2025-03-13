use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(id: impl Into<Uuid>) -> Self {
        Self(id.into())
    }
}

impl From<UserId> for Uuid {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
