use axum::{
    Json, 
    extract::State, 
    http::HeaderMap
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;

// --- PERBAIKAN DI SINI ---
// Sebelumnya mungkin tertulis: use crate::TransferUseCase;
// Yang benar adalah path lengkapnya:
use crate::services::transfer::TransferUseCase;

use crate::domain::models::AccountId;

#[derive(Deserialize)]
pub struct TransferRequest {
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub amount: Decimal,
}

pub async fn transfer_handler(
    headers: HeaderMap,
    State(service): State<Arc<TransferUseCase>>,
    Json(payload): Json<TransferRequest>,
) -> Result<Json<String>, String> {
    
    // 1. Ambil Header Idempotency-Key
    let idempotency_key = headers
        .get("X-Idempotency-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or("Missing header: X-Idempotency-Key")? 
        .to_string();

    let from = AccountId(payload.from_account);
    let to = AccountId(payload.to_account);

    // 2. Panggil service dengan Key
    match service.execute(idempotency_key, from, to, payload.amount).await {
        Ok(msg) => Ok(Json(msg)),
        Err(e) => Err(format!("Transfer Failed: {}", e)),
    }
}