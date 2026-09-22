use thiserror::Error;
use uuid::Uuid;

use crate::store::{GlobalSequence, StreamVersion};

pub trait StreamId: From<Uuid> + Clone + Send + Sync {
    fn key(&self) -> Uuid;
}

pub struct Envelope<E: Event, ID: StreamId> {
    pub event: E,
    pub stream_id: ID,
    pub stream_version: StreamVersion,
    pub event_kind: &'static str,
}

pub trait Event: Send + Sync {
    fn kind(&self) -> &'static str;
}

pub enum ExpectedVersion {
    NoStream,
    Any,
    At(u64),
}

pub enum AppendOutcome {
    Appended { first_sequence: u64 },
    Conflict { actual: u64 }, // caller decides: retry or bubble up
}
