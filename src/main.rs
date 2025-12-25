use axum::{routing::post, Router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use dotenv::dotenv;

// Mendaftarkan modul-modul (folder)
mod domain;
mod ports;
mod services;
mod adapters;
mod tests;

// --- BAGIAN IMPORTS YANG PENTING ---
// Pastikan baris-baris ini ada:
use crate::adapters::persistence::postgres_repo::PostgresBankingRepository;
use crate::services::transfer::TransferUseCase; // <--- INI PERBAIKANNYA
use crate::adapters::api::handler::transfer_handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load Environment Variables (.env)
    dotenv().ok();
    
    // Logging sederhana
    tracing_subscriber::fmt::init();
    println!("🚀 Starting Banking System...");

    // 2. Setup Database Connection Pool
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    println!("✅ Database connected successfully.");

    // 3. Dependency Injection (Merakit Aplikasi)
    
    // Layer 1: Repository (Infrastructure)
    let repo = Arc::new(PostgresBankingRepository::new(pool));

    // Layer 2: Service (Application Logic)
    // Kita menyuntikkan repository ke dalam service
    let transfer_service = Arc::new(TransferUseCase::new(repo));

    // 4. Setup Router & State
    // Kita menyuntikkan service ke dalam handler Axum
    let app = Router::new()
        .route("/transfer", post(transfer_handler))
        .with_state(transfer_service);

    // 5. Start Server
    // Pastikan port tidak bentrok (bisa ganti 3001 jika 3000 masih nyangkut)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await?;
    println!("🔥 Server listening on port 3003");
    axum::serve(listener, app).await?;

    Ok(())
}