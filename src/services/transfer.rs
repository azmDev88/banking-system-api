use std::sync::Arc;
use anyhow::{Result, anyhow};
use rust_decimal::Decimal;

// Import hanya AccountId, hapus Account agar warning hilang
use crate::domain::models::AccountId;
use crate::ports::repository::BankingRepository;

// --- BAGIAN YANG HILANG (STRUCT DEFINITION) ---
// Ini wajib ada sebelum block 'impl'
pub struct TransferUseCase {
    repo: Arc<dyn BankingRepository>,
}

// --- IMPLEMENTASI LOGIC ---
impl TransferUseCase {
    // Constructor
    pub fn new(repo: Arc<dyn BankingRepository>) -> Self {
        Self { repo }
    }

    // Logic Utama Transfer dengan Idempotency
    pub async fn execute(
        &self, 
        idempotency_key: String, 
        from: AccountId, 
        to: AccountId, 
        amount: Decimal
    ) -> Result<String> {
        
        // 1. Mulai Transaksi Database
        let mut tx = self.repo.begin_tx().await?;

        // 2. CEK IDEMPOTENCY (Anti-Double Spending)
        if let Some(previous_response) = tx.get_idempotency_key(&idempotency_key).await? {
            println!("♻️ Idempotency Hit! Mengembalikan respon lama.");
            return Ok(previous_response); 
        }

        // 3. Sorting ID untuk mencegah Deadlock (Standard Banking)
        let (first_id, second_id) = if from.0 < to.0 {
            (from, to)
        } else {
            (to, from)
        };

        // 4. Lock Akun di Database (SELECT FOR UPDATE)
        let _acc1 = tx.get_account_for_update(first_id).await?;
        let _acc2 = tx.get_account_for_update(second_id).await?;

        // 5. Ambil Data Akun Asli untuk diproses
        let mut sender = tx.get_account_for_update(from).await?;
        let mut receiver = tx.get_account_for_update(to).await?;

        // 6. Domain Logic (Mutasi Saldo di Memory Rust)
        // Kita beri tipe eksplisit |e: String| untuk memberitahu compiler tipe errornya
        sender.debit(amount).map_err(|e: String| anyhow!(e))?;
        receiver.credit(amount).map_err(|e: String| anyhow!(e))?;

        // 7. Simpan Perubahan ke Database
        tx.update_account(&sender).await?;
        tx.update_account(&receiver).await?;
        
        // 8. Catat Audit Log
        tx.save_transaction_log(from, to, amount).await?;

        // 9. Simpan Idempotency Key (Tanda bahwa transaksi ini sukses)
        let success_msg = "Transfer Successful".to_string();
        tx.save_idempotency_key(&idempotency_key, 200, &success_msg).await?;

        // 10. Commit Transaksi (Permanen)
        tx.commit().await?;

        Ok(success_msg)
    }
}