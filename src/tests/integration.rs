use crate::domain::models::{AccountId};
use crate::services::transfer::TransferUseCase;
use crate::adapters::persistence::postgres_repo::PostgresBankingRepository;
use sqlx::postgres::PgPoolOptions;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::sync::Arc;
use uuid::Uuid;
use dotenv::dotenv;

// --- HELPER: Setup Database Bersih ---
// Fungsi ini dijalankan sebelum setiap test case agar data selalu fresh
async fn setup_test_db() -> (Arc<TransferUseCase>, AccountId, AccountId) {
    // 1. Load Environment Variables
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    
    // 2. Konek Database
    let pool = PgPoolOptions::new()
        .max_connections(100)// Butuh koneksi agak banyak untuk test concurrency
        .connect(&db_url)
        .await
        .expect("Failed to connect DB");

    // 3. SAPU BERSIH TABEL (Reset State)
    sqlx::query("TRUNCATE TABLE transaction_logs, idempotency_keys, accounts")
        .execute(&pool).await.unwrap();

    // 4. Seed Data Awal
    let id_a = AccountId(Uuid::new_v4());
    let id_b = AccountId(Uuid::new_v4());

    // Adam: Saldo 1 Juta
    sqlx::query("INSERT INTO accounts (id, owner_name, balance, currency, version) VALUES ($1, 'Adam Test', 1000000, 'IDR', 1)")
        .bind(id_a.0).execute(&pool).await.unwrap();
    
    // Budi: Saldo 0
    sqlx::query("INSERT INTO accounts (id, owner_name, balance, currency, version) VALUES ($1, 'Budi Test', 0, 'IDR', 1)")
        .bind(id_b.0).execute(&pool).await.unwrap();

    // 5. Setup Service
    let repo = Arc::new(PostgresBankingRepository::new(pool));
    let service = Arc::new(TransferUseCase::new(repo));

    (service, id_a, id_b)
}

// --- HELPER: Cek Saldo Langsung ke DB ---
async fn get_balance(id: AccountId) -> Decimal {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    
    let row: (Decimal,) = sqlx::query_as("SELECT balance FROM accounts WHERE id = $1")
        .bind(id.0)
        .fetch_one(&pool).await.unwrap();
    row.0
}

// ==========================================
// TEST CASE 1: SUCCESS STORY (HAPPY PATH)
// ==========================================
#[tokio::test]
async fn test_transfer_success() {
    let (service, from, to) = setup_test_db().await;
    
    // Action: Transfer 50rb
    let result = service.execute(
        "key-sukses-1".to_string(), 
        from, 
        to, 
        dec!(50000)
    ).await;

    // Assert Logic
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Transfer Successful");

    // Assert Database: Adam sisa 950rb, Budi ada 50rb
    assert_eq!(get_balance(from).await, dec!(950000));
    assert_eq!(get_balance(to).await, dec!(50000));
}

// ==========================================
// TEST CASE 2: ERROR CASE (SALDO KURANG)
// ==========================================
#[tokio::test]
async fn test_transfer_insufficient_funds() {
    let (service, from, to) = setup_test_db().await;

    // Action: Transfer 5 Juta (Padahal cuma punya 1 Juta)
    let result = service.execute(
        "key-gagal-1".to_string(), 
        from, 
        to, 
        dec!(5000000)
    ).await;

    // Assert Logic
    assert!(result.is_err());
    let err_msg = result.err().unwrap().to_string();
    // Pastikan pesan errornya benar (bukan error database connection dll)
    assert!(err_msg.contains("Insufficient funds")); 

    // Assert Database: Saldo TIDAK BERUBAH
    assert_eq!(get_balance(from).await, dec!(1000000));
}

// ==========================================
// TEST CASE 3: IDEMPOTENCY (ANTI DUPLIKAT)
// ==========================================
#[tokio::test]
async fn test_idempotency_retry() {
    let (service, from, to) = setup_test_db().await;
    let key = "key-unik-123".to_string();

    // Request 1: Sukses
    let res1 = service.execute(key.clone(), from, to, dec!(100000)).await;
    assert!(res1.is_ok());

    // Request 2: RETRY dengan KEY SAMA (Simulasi User tekan tombol 2x)
    let res2 = service.execute(key.clone(), from, to, dec!(100000)).await;
    assert!(res2.is_ok()); // Tetap return OK (agar user tidak bingung)

    // Assert Database: Saldo harusnya berkurang CUMA SEKALI (900rb), bukan 800rb
    assert_eq!(get_balance(from).await, dec!(900000));
}

// ==========================================
// TEST CASE 4: CONCURRENCY / RACE CONDITION
// (Simulasi serangan request bersamaan)
// ==========================================
#[tokio::test]
async fn test_concurrent_transfers() {
    let (service, from, to) = setup_test_db().await;
    
    // Kita akan buat 5 "User" menembak API secara BERSAMAAN
    // Masing-masing transfer 10.000
    let mut handles = vec![];

    for i in 0..5 {
        let svc = service.clone(); // Clone Arc (murah & thread safe)
        let key = format!("concurrent-key-{}", i); // Key harus beda tiap request
        let f = from;
        let t = to;

        let handle = tokio::spawn(async move {
            svc.execute(key, f, t, dec!(10000)).await
        });
        handles.push(handle);
    }

    // Tunggu semua "User" selesai
    for h in handles {
        let _ = h.await;
    }

    // HITUNG HASIL:
    // Total transfer: 5 user * 10.000 = 50.000
    // Saldo Awal Adam: 1.000.000
    // Saldo Akhir Harapan: 950.000
    
    let final_balance_from = get_balance(from).await;
    let final_balance_to = get_balance(to).await;

    // Jika Locking Database (FOR UPDATE) kita gagal, angka ini pasti salah.
    assert_eq!(final_balance_from, dec!(950000), "Race condition detected! Saldo Pengirim tidak sinkron.");
    assert_eq!(final_balance_to, dec!(50000), "Race condition detected! Saldo Penerima tidak sinkron.");
}