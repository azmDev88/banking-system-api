use crate::domain::models::{Account, AccountId};
use rust_decimal_macros::dec;
use uuid::Uuid;

#[test]
fn test_debit_success() {
    // 1. Setup: Akun dengan saldo 100
    let mut account = Account {
        id: AccountId(Uuid::new_v4()),
        balance: dec!(100),
        version: 1,
    };

    // 2. Action: Kurangi 50
    let result = account.debit(dec!(50));

    // 3. Assert: Berhasil & Sisa 50
    assert!(result.is_ok());
    assert_eq!(account.balance, dec!(50));
}

#[test]
fn test_debit_insufficient_funds() {
    // 1. Setup: Akun saldo 100
    let mut account = Account {
        id: AccountId(Uuid::new_v4()),
        balance: dec!(100),
        version: 1,
    };

    // 2. Action: Kurangi 150 (Lebih besar dari saldo)
    let result = account.debit(dec!(150));

    // 3. Assert: Harus Error
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Insufficient funds");
    assert_eq!(account.balance, dec!(100)); // Saldo tidak boleh berubah
}

#[test]
fn test_credit_success() {
    let mut account = Account {
        id: AccountId(Uuid::new_v4()),
        balance: dec!(100),
        version: 1,
    };

    let _ = account.credit(dec!(50));
    assert_eq!(account.balance, dec!(150));
}

#[test]
fn test_negative_amount() {
    let mut account = Account {
        id: AccountId(Uuid::new_v4()),
        balance: dec!(100),
        version: 1,
    };

    // Tidak boleh transfer nilai minus
    let result = account.credit(dec!(-10));
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Amount must be positive");
}