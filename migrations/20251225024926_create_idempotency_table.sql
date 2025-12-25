CREATE TABLE idempotency_keys (
    idempotency_key VARCHAR(100) PRIMARY KEY,
    response_status SMALLINT NOT NULL, -- Misal: 200, 400
    response_body TEXT,                -- JSON hasil respon sebelumnya
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);-- Add migration script here
