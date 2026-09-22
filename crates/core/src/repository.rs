use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;

use crate::{
    aggregate::{Aggregate, Apply, BootstrapError},
    command::{CommandError, Handle},
    event::{AppendOutcome, Envelope, ExpectedVersion, StreamId},
    store::{EventStore, StoreError},
};

// #[async_trait]
// pub trait Repository<A: Aggregate> {
//     pub async fn load(
//         &self,
//         id: &A::Id,
//     ) -> Result
// }

#[derive(Debug)]
pub struct Repository<A: Aggregate, S: EventStore> {
    store: Arc<S>,
    _aggregate: std::marker::PhantomData<fn() -> A>, // zero-sized, covariant
}

#[derive(Error, Debug)]
pub enum ExecuteError<D: std::error::Error, S: std::error::Error> {
    #[error("domain error: {0}")]
    Domain(D),
    #[error("store error: {0}")]
    Store(StoreError<S>),
    #[error("concurrency conflict")]
    Concurrency,
    #[error("consistency error: state generation does not match last event version")]
    Consistency,
    #[error("aggregate not initialized by bootstrap event")]
    NotInitialized,
}

impl<A, S> Repository<A, S>
where
    S: EventStore,
    A: Aggregate + 'static,
    A::Id: StreamId,
{
    pub fn new(store: Arc<S>) -> Self {
        Self {
            store,
            _aggregate: std::marker::PhantomData,
        }
    }

    pub async fn load(
        &self,
        id: &A::Id,
    ) -> Result<(Option<A>, ExpectedVersion), ExecuteError<A::Error, S::Error>> {
        let envelopes = self
            .store
            .load::<A>(id)
            .await
            .map_err(ExecuteError::Store)?;

        self.fold(envelopes)
    }

    // pub fn load_with_snapshot(
    //     &self,
    //     id: &D::Id,
    //     snap: &dyn SnapshotStore,
    // ) -> Result<Loaded<D::State>, StoreError>
    // where
    //     D::State: Snapshot,
    // {
    //     if let Some(record) = snap.load(id)? {
    //         if let Ok(state) = D::State::from_payload(record.payload) {
    //             // replay only the tail after the snapshot's version
    //             let tail = self.store.load_after(&id.key(), record.version)?;
    //             return self.fold_from(state, record.version, tail);
    //         }
    //         // payload unreadable (schema drift, corruption):
    //         // snapshot is only a cache — fall through to full replay
    //     }
    //     self.load(id)
    // }

    pub async fn execute<C, R>(
        &self,
        id: &A::Id,
        cmd: C,
    ) -> Result<R, ExecuteError<A::Error, S::Error>>
    where
        A: Handle<C, R>,
    {
        // TODO: load with snapshot, if available
        let (state, version) = self.load(id).await?;

        let (result, events) = if let Some(state) = state {
            state.handle(cmd).map_err(|e| match e {
                CommandError::Domain(e) => ExecuteError::Domain(e),
                _ => ExecuteError::Consistency,
            })?
        } else {
            A::create(cmd).unwrap()
        };

        // let (state, expected) = match loaded {
        //     Loaded::Absent => (None, ExpectedVersion::NoStream),
        //     Loaded::Present(s) => {
        //         let expected_version = ExpectedVersion::At(s.generation());
        //
        //         (Some(s), expected_version)
        //     }
        // };

        // let events = A::handle(&state, cmd).map_err(ExecuteError::Domain)?;
        self.store
            .append::<A>(id, version, &events)
            .await
            .map(|_| result)
            .map_err(ExecuteError::Store)
    }

    /// The fold itself — private, never exposed to users.
    fn fold(
        &self,
        envelopes: Vec<Envelope<A::Event, A::Id>>,
    ) -> Result<(Option<A>, ExpectedVersion), ExecuteError<A::Error, S::Error>> {
        if envelopes.is_empty() {
            return Ok((None, ExpectedVersion::NoStream));
        }

        let version = envelopes
            .last()
            .expect("should never happen")
            .stream_version
            .0;

        let mut acc: Option<A> = None;
        for (i, e) in envelopes.into_iter().enumerate() {
            if e.stream_version.0 != i as u64 {
                return Err(ExecuteError::Consistency);
            }

            acc = match acc {
                Some(mut a) => {
                    a.evolve(e.event).map_err(ExecuteError::Domain)?;
                    Some(a)
                }
                None => Some(A::init(e.event).map_err(|_| ExecuteError::NotInitialized)?),
            };
        }
        Ok((acc, ExpectedVersion::At(version)))
    }
}
