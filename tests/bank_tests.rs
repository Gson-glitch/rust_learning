use rust_learning::banking::account::{AccountType, BankAccount};

#[test]
fn test_bank_deposit() {
    let mut bank_acc: BankAccount = BankAccount::new("Gson", AccountType::Checking);
    bank_acc.deposit(100.0);
    assert_eq!(
        bank_acc.check_balance(),
        100.0,
        "Bank balance should be 100.0 after deposit"
    );
}
