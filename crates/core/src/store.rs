use std::pin::Pin;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    aggregate::Aggregate,
    error::AggregateError,
    event::{AppendOutcome, Envelope, Event as _, ExpectedVersion, StreamId},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalSequence(pub u64);
pub struct StreamVersion(pub u64);

#[derive(Error, Debug)]
pub enum StoreError<S: std::error::Error> {
    #[error("storage error: {0}")]
    Storage(S),
    #[error("projection error: {0}")]
    Projection(#[from] ProjectionError),
}

#[derive(Debug, Error)]
#[error("transactional projection failed: {context}")]
pub struct ProjectionError {
    context: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl ProjectionError {
    pub fn new(
        context: impl Into<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    ) -> Self {
        Self {
            context: context.into(),
            source,
        }
    }
}

#[async_trait]
pub trait EventStore: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn load<A>(
        &self,
        id: &A::Id,
    ) -> Result<Vec<Envelope<A::Event, A::Id>>, StoreError<Self::Error>>
    where
        A: Aggregate,
        A::Id: StreamId,
        A::Event: DeserializeOwned;

    async fn append<A>(
        &self,
        id: &A::Id,
        expected: ExpectedVersion,
        events: &[A::Event],
    ) -> Result<(), StoreError<Self::Error>>
    where
        A: Aggregate + 'static,
        A::Id: StreamId,
        A::Event: Serialize;
    // async fn load_after(&self, stream: &StreamKey, after: u64) -> ...;
    // async fn subscribe(&self, after: GlobalSequence) -> ...;   // for projections/PMs

    fn wrap<A>(
        &self,
        events: &[A::Event],
        id: &A::Id,
        first_sequence: ExpectedVersion,
    ) -> Vec<Envelope<A::Event, A::Id>>
    where
        A: Aggregate,
        A::Id: StreamId,
    {
        let first_sequence = match first_sequence {
            ExpectedVersion::NoStream => 0,
            ExpectedVersion::Any => panic!("Cannot wrap events with ExpectedVersion::Any"),
            ExpectedVersion::At(seq) => seq,
        };
        events
            .iter()
            .cloned()
            .zip(first_sequence..)
            .map(|(event, seq)| Envelope {
                stream_id: id.clone(),
                stream_version: StreamVersion(seq),
                event_kind: event.kind(),
                event,
            })
            .collect::<Vec<_>>()
    }
}
