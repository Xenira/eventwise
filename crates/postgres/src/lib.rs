use std::{any::TypeId, collections::HashMap};

use async_trait::async_trait;
use eventwise_core::{
    aggregate::Aggregate,
    event::{AppendOutcome, Envelope, Event, ExpectedVersion, StreamId},
    store::{EventStore, GlobalSequence, ProjectionError, StoreError, StreamVersion},
};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Postgres, Transaction};

pub struct PostgresEventStore {
    pool: sqlx::PgPool,
    projections: HashMap<TypeId, Vec<Box<dyn ErasedTxProjection>>>,
}

impl PostgresEventStore {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool,
            projections: HashMap::new(),
        }
    }

    pub fn register_projection<A, P>(&mut self, projection: P)
    where
        A: Aggregate + 'static,
        A::Event: Serialize + DeserializeOwned,
        A::Id: StreamId,
        P: TxProjection<A> + 'static,
    {
        let boxed: Box<dyn TxProjection<A>> = Box::new(projection);
        self.projections
            .entry(TypeId::of::<A>())
            .or_insert_with(Vec::new)
            .push(Box::new(boxed));
    }

    async fn persist_events<A: Aggregate>(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        events: &[EnvelopeErased],
    ) -> Result<(), sqlx::Error> {
        for event in events {
            sqlx::query!(
                r#"
                INSERT INTO events (aggregate_kind, aggregate_id, sequence, event_kind, data)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                A::KIND,
                event.stream_id,
                event.sequence,
                event.event_kind,
                event.payload
            )
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }
}

#[async_trait]
impl EventStore for PostgresEventStore {
    type Error = sqlx::Error;

    async fn load<A>(
        &self,
        id: &A::Id,
    ) -> Result<Vec<Envelope<A::Event, A::Id>>, StoreError<Self::Error>>
    where
        A: Aggregate,
        A::Id: StreamId,
        A::Event: DeserializeOwned,
    {
        let rows = sqlx::query!(
            r#"
            SELECT sequence, event_kind, data
            FROM events
            WHERE aggregate_kind = $1 AND aggregate_id = $2
            ORDER BY sequence ASC
            "#,
            A::KIND,
            id.key()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(StoreError::Storage)?;

        rows.into_iter()
            .map(|row| {
                let event: A::Event = serde_json::from_value(row.data).map_err(|e| {
                    ProjectionError::new("Failed to deserialize event", Some(Box::new(e)))
                })?;
                Ok(Envelope {
                    stream_id: id.clone(),
                    stream_version: StreamVersion(row.sequence.try_into().map_err(|e| {
                        ProjectionError::new(
                            "Failed to convert sequence to stream version",
                            Some(Box::new(e)),
                        )
                    })?),
                    event_kind: event.kind(),
                    event,
                })
            })
            .collect::<Result<Vec<_>, StoreError<Self::Error>>>()
    }

    async fn append<A>(
        &self,
        id: &A::Id,
        expected: ExpectedVersion,
        events: &[A::Event],
    ) -> Result<(), StoreError<Self::Error>>
    where
        A: Aggregate + 'static,
        A::Id: StreamId,
        A::Event: Serialize,
    {
        let mut tx = self.pool.begin().await.map_err(StoreError::Storage)?;
        let envelopes: Vec<Envelope<A::Event, A::Id>> = self.wrap::<A>(events, id, expected);
        let erased: Vec<EnvelopeErased> = envelopes
            .iter()
            .map(|e| EnvelopeErased {
                stream_id: e.stream_id.key(),
                sequence: e.stream_version.0.try_into().unwrap(),
                payload: serde_json::to_value(&e.event).unwrap(),
                event_kind: e.event_kind,
            })
            .collect();

        self.persist_events::<A>(&mut tx, &erased)
            .await
            .map_err(StoreError::Storage)?;

        for projection in self
            .projections
            .get(&TypeId::of::<A>())
            .unwrap_or(&Vec::new())
        {
            projection.apply_erased(&mut tx, &erased).await?;
        }

        tx.commit().await.map_err(StoreError::Storage)
        // Ok(AppendOutcome::Appended { first_sequence: 0 }) // TODO: return the actual first sequence number
    }
}

#[async_trait]
pub trait TxProjection<A: Aggregate>: Send + Sync {
    /// Which stream kinds this projection participates in.
    fn kinds(&self) -> &'static [&'static str];

    /// Runs INSIDE the append transaction. Failure aborts the append.
    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'static, Postgres>,
        events: &[Envelope<A::Event, A::Id>],
    ) -> Result<(), ProjectionError>;
}

#[async_trait]
trait ErasedTxProjection: Send + Sync {
    async fn apply_erased(
        &self,
        tx: &mut sqlx::Transaction<'static, Postgres>,
        raw: &[EnvelopeErased],
    ) -> Result<(), ProjectionError>;
}

pub(crate) struct EnvelopeErased {
    pub stream_id: uuid::Uuid,
    pub sequence: i64,
    pub payload: serde_json::Value,
    pub event_kind: &'static str,
}

#[async_trait::async_trait]
impl<A: Aggregate> ErasedTxProjection for Box<dyn TxProjection<A>>
where
    A::Event: DeserializeOwned + Serialize,
    A::Id: StreamId,
{
    async fn apply_erased(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        raw: &[EnvelopeErased],
    ) -> Result<(), ProjectionError> {
        // Decode here, once, at the erasure boundary:
        let typed: Vec<Envelope<A::Event, A::Id>> = raw
            .iter()
            .map(|r| {
                Ok(Envelope {
                    event: serde_json::from_value(r.payload.clone()).map_err(|e| {
                        ProjectionError::new("Failed to serialize event", Some(Box::new(e)))
                    })?,
                    stream_id: A::Id::from(r.stream_id),
                    stream_version: StreamVersion(r.sequence.try_into().map_err(|e| {
                        ProjectionError::new(
                            "Failed to convert sequence to stream version",
                            Some(Box::new(e)),
                        )
                    })?),
                    event_kind: r.event_kind,
                })
            })
            .collect::<Result<_, ProjectionError>>()?;
        // From here on, the projection sees only typed envelopes:
        (**self).apply(tx, &typed).await
    }
}
