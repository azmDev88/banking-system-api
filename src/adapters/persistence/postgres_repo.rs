use anyhow::{anyhow, Result};
use async_trait::async_trait;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};

// Import Domain Models
use crate::domain::models::{Account, AccountId};
// Import Interface (Ports)
use crate::ports::repository::{BankingRepository, TransactionPort};
// Import Entity Database (Struct perantara)
use super::entities::AccountSqlRow;

// ================================================================
// 1. Repository Implementation (Entry Point)
// ================================================================

// Struct harus 'pub' agar bisa dipanggil di main.rs
pub struct PostgresBankingRepository {
    pool: PgPool,
}

impl PostgresBankingRepository {
    // Constructor harus 'pub'
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BankingRepository for PostgresBankingRepository {
    // Memulai transaksi database baru
    async fn begin_tx(&self) -> Result<Box<dyn TransactionPort>> {
        let tx = self.pool.begin().await?;
        Ok(Box::new(PostgresTransaction { tx }))
    }
}

// ================================================================
// 2. Transaction Implementation (Logic Query Sebenarnya)
// ================================================================

// Wrapper untuk SQLx Transaction
pub struct PostgresTransaction<'c> {
    tx: Transaction<'c, Postgres>,
}

#[async_trait]
impl<'c> TransactionPort for PostgresTransaction<'c> {
    // ... di dalam impl TransactionPort ...

    async fn get_idempotency_key(&mut self, key: &str) -> Result<Option<String>> {
        // Kita lock row ini juga agar tidak ada 2 request bersamaan yg insert key sama
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT response_body FROM idempotency_keys WHERE idempotency_key = $1 FOR UPDATE"
        )
        .bind(key)
        .fetch_optional(&mut *self.tx)
        .await?;

        Ok(row.map(|r| r.0))
    }

    async fn save_idempotency_key(&mut self, key: &str, status: u16, body: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO idempotency_keys (idempotency_key, response_status, response_body) VALUES ($1, $2, $3)"
        )
        .bind(key)
        .bind(status as i16)
        .bind(body)
        .execute(&mut *self.tx)
        .await?;
        
        Ok(())
    }

// ... method commit dll ...
    
    // MENGAMBIL DATA DENGAN PESSIMISTIC LOCK (SELECT ... FOR UPDATE)
    async fn get_account_for_update(&mut self, id: AccountId) -> Result<Account> {
        let row = sqlx::query_as::<_, AccountSqlRow>(
            r#"
            SELECT id, balance, currency, version 
            FROM accounts 
            WHERE id = $1 
            FOR UPDATE
            "#
        )
        .bind(id.0) // Mengambil nilai UUID dari struct AccountId
        .fetch_optional(&mut *self.tx)
        .await?;

        match row {
            Some(r) => Ok(r.to_domain()),
            None => Err(anyhow!("Account not found with ID: {}", id.0)),
        }
    }

    // UPDATE SALDO & VERSION (Optimistic Concurrency Check)
    async fn update_account(&mut self, account: &Account) -> Result<()> {
        let result = sqlx::query(
            r#"
            UPDATE accounts 
            SET balance = $1, version = version + 1, updated_at = NOW()
            WHERE id = $2
            "#
        )
        .bind(account.balance)
        .bind(account.id.0)
        .execute(&mut *self.tx)
        .await?;

        if result.rows_affected() == 0 {
            // Jika 0 rows, artinya ID salah ATAU data sudah dihapus orang lain
            return Err(anyhow!("Failed to update account. Concurrency error or ID not found."));
        }
        Ok(())
    }

    // SIMPAN LOG TRANSAKSI (AUDIT TRAIL)
    async fn save_transaction_log(&mut self, from: AccountId, to: AccountId, amount: Decimal) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO transaction_logs (id, from_account_id, to_account_id, amount, created_at)
            VALUES (uuid_generate_v4(), $1, $2, $3, NOW())
            "#
        )
        .bind(from.0)
        .bind(to.0)
        .bind(amount)
        .execute(&mut *self.tx)
        .await?;
        
        Ok(())
    }


    

    // COMMIT TRANSAKSI (FINALISASI)
    async fn commit(self: Box<Self>) -> Result<()> {
        self.tx.commit().await.map_err(|e| anyhow!(e))
    }
}