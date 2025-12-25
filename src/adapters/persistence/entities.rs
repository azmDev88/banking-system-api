use sqlx::FromRow;
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::domain::models::{Account, AccountId};

#[derive(FromRow)]
pub struct AccountSqlRow {
    pub id: Uuid,
    pub balance: Decimal,
    //pub currency: String,
    pub version: i32,
}

impl AccountSqlRow {
    // Helper untuk mengubah struct SQL menjadi struct Domain
    pub fn to_domain(self) -> Account {
        Account {
            id: AccountId(self.id),
            balance: self.balance,
            version: self.version,
        }
    }
}