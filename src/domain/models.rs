use rust_decimal::Decimal;
use uuid::Uuid;

// TAMBAHKAN 'pub' DI DEPAN
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(pub Uuid); // <-- pub struct DAN pub field (pub Uuid)

// TAMBAHKAN 'pub' DI DEPAN
#[derive(Debug, Clone)]
pub struct Account {
    pub id: AccountId,    // <-- pub field
    pub balance: Decimal, // <-- pub field
    pub version: i32,     // <-- pub field
}

impl Account {
    // Function juga harus pub agar bisa dipanggil Service
    pub fn debit(&mut self, amount: Decimal) -> Result<(), String> {
        // ... kode lama ...
        if amount <= Decimal::ZERO { return Err("Amount must be positive".to_string()); }
        if self.balance < amount { return Err("Insufficient funds".to_string()); }
        self.balance -= amount;
        Ok(())
    }

    pub fn credit(&mut self, amount: Decimal) -> Result<(), String> {
        // ... kode lama ...
        if amount <= Decimal::ZERO { return Err("Amount must be positive".to_string()); }
        self.balance += amount;
        Ok(())
    }
}