-- Infrastructure metadata for the trading platform database.
-- This milestone records schema purpose only.
-- OMS, risk, execution, and broker identity tables are out of scope.

CREATE TABLE infrastructure_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO infrastructure_metadata (key, value)
VALUES ('schema_purpose', 'infrastructure-metadata-only');
