use std::fmt::Display;

use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

use crate::event::{Event, StreamId};

#[derive(Debug)]
pub struct BootstrapError<E>(pub E)
where
    E: std::fmt::Debug;

impl<E> Display for BootstrapError<E>
where
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bootstrap error: {:?}", self.0)
    }
}
impl<E> std::error::Error for BootstrapError<E> where E: std::fmt::Debug {}

pub trait Aggregate: Sized {
    type Event: Event + Serialize + DeserializeOwned + Clone + Send + Sync + std::fmt::Debug;
    type Error: std::error::Error;
    type Id: StreamId;
    const KIND: &'static str;

    fn create(event: Self::Event) -> Result<Self, BootstrapError<Self::Event>>;
    // fn create(event: &Self::Event) -> Result<Self::State, Self::Error>;
    fn evolve(&mut self, event: Self::Event) -> Result<(), Self::Error>;
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("aggregate not initialized by bootstrap event")]
    BootstrapError,
}

pub trait Apply<E>: Sized {
    fn apply(&mut self, event: E);
}

pub trait Create<E>: Aggregate
where
    E: std::fmt::Debug,
{
    fn bootstrap(event: E) -> Self;
}
