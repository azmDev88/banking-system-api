-- Add migration script here
-- Pastikan ekstensi UUID aktif
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Tabel Accounts dengan struktur High Precision & Concurrency Control
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_name VARCHAR(100) NOT NULL,
    
    -- DECIMAL(20, 4) wajib untuk uang. JANGAN FLOAT.
    balance NUMERIC(20, 4) NOT NULL DEFAULT 0.0000,
    
    currency VARCHAR(3) NOT NULL DEFAULT 'IDR',
    
    -- Kolom sakti untuk Optimistic Locking
    version INTEGER NOT NULL DEFAULT 1,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Database-level constraint: Saldo minus dilarang keras
    CONSTRAINT balance_non_negative CHECK (balance >= 0)
);

-- Index agar pencarian nasabah cepat
CREATE INDEX idx_accounts_owner ON accounts(owner_name);

-- Tabel Audit Log (Penting untuk tracking mutasi)
CREATE TABLE transaction_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    from_account_id UUID NOT NULL REFERENCES accounts(id),
    to_account_id UUID NOT NULL REFERENCES accounts(id),
    amount NUMERIC(20, 4) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);