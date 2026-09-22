use std::error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AggregateError<T: error::Error> {
    #[error("{0}")]
    UserError(T),
    #[error("aggregate conflict")]
    Conflict,
    #[error("Failure in storage backend: {0}")]
    StorageError(Box<dyn error::Error + Send + Sync + 'static>),
    #[error("Failed to deserialize persisted object: {0}")]
    DeserializationError(Box<dyn error::Error + Send + Sync + 'static>),
    #[error("{0}")]
    UnexpectedError(Box<dyn error::Error + Send + Sync + 'static>),
}
