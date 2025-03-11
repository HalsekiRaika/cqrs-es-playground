#[derive(Debug, thiserror::Error)]
#[error("unrecoverable error")]
pub struct UnrecoverableError;

#[derive(Debug, thiserror::Error)]
#[error("command rejected")]
pub struct CommandRejected;
