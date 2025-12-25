use async_trait::async_trait;
use crate::domain::models::{Account, AccountId};
use rust_decimal::Decimal;
use anyhow::Result;

// 1. Interface Utama (Factory Transaction)
#[async_trait]
pub trait BankingRepository: Send + Sync {
    async fn begin_tx(&self) -> Result<Box<dyn TransactionPort>>;
}

// 2. Interface Transaksi (Kontrak Kerja)
// Error Anda terjadi karena fungsi idempotency belum ada di sini
#[async_trait]
pub trait TransactionPort: Send {
    // Fungsi Lama
    async fn get_account_for_update(&mut self, id: AccountId) -> Result<Account>;
    async fn update_account(&mut self, account: &Account) -> Result<()>;
    async fn save_transaction_log(&mut self, from: AccountId, to: AccountId, amount: Decimal) -> Result<()>;
    
    // --- FUNGSI BARU (WAJIB DITAMBAHKAN) ---
    // Inilah yang dicari oleh compiler di postgres_repo.rs
    async fn get_idempotency_key(&mut self, key: &str) -> Result<Option<String>>;
    async fn save_idempotency_key(&mut self, key: &str, status: u16, body: &str) -> Result<()>;
    
    // Commit
    async fn commit(self: Box<Self>) -> Result<()>;
}