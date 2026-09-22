CREATE TABLE events (
  global_id      BIGSERIAL PRIMARY KEY,
  aggregate_kind TEXT                        NOT NULL,
  aggregate_id   UUID                        NOT NULL,
  sequence       BIGINT CHECK (sequence > 0) NOT NULL,
  event_kind     TEXT                        NOT NULL,
  data           JSON                        NOT NULL,
  created_at     TIMESTAMPTZ                 NOT NULL DEFAULT NOW(),
  UNIQUE (aggregate_kind, aggregate_id, sequence)
);

CREATE INDEX idx_events_aggregate ON events (aggregate_kind, aggregate_id);
