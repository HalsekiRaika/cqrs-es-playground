#[derive(Debug, thiserror::Error)]
#[error("unrecoverable error")]
pub struct UnrecoverableError;

#[derive(Debug, thiserror::Error)]
#[error("command rejected")]
pub struct CommandRejected;

#[derive(Debug, thiserror::Error)]
#[error("sending into a closed channel")]
pub struct ChannelDropped;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ReadError(Box<dyn std::error::Error + Sync + Send>);

impl From<sqlx::Error> for ReadError {
    fn from(err: sqlx::Error) -> Self {
        Self(Box::new(err))
    }
}

impl From<DeserializeError> for ReadError {
    fn from(err: DeserializeError) -> Self {
        Self(err.0)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct WriteError(Box<dyn std::error::Error + Sync + Send>);

impl From<sqlx::Error> for WriteError {
    fn from(err: sqlx::Error) -> Self {
        Self(Box::new(err))
    }
}

impl From<SerializeError> for WriteError {
    fn from(err: SerializeError) -> Self {
        Self(err.0)
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct SerializeError(Box<dyn std::error::Error + Sync + Send>);

impl<T> From<T> for SerializeError 
where
    T: serde::ser::Error + 'static + Sync + Send 
{
    fn from(err: T) -> Self {
        Self(Box::new(err))
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct DeserializeError(Box<dyn std::error::Error + Sync + Send>);

impl<T> From<T> for DeserializeError 
where
    T: serde::de::Error + 'static + Sync + Send 
{
    fn from(err: T) -> Self {
        Self(Box::new(err))
    }
}