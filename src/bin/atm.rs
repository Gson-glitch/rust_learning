// We use the package name to find our library
use rust_learning::banking::account::{AccountType, BankAccount};
use rust_learning::banking::ops;

fn main() {
    let mut gson_acc: BankAccount = BankAccount::new("Gson", AccountType::Checking);
    gson_acc.deposit(100.50);

    println!(
        "User: {}, Balance: ${}",
        gson_acc.holder,
        gson_acc.check_balance()
    );

    // gson_acc.balance = 1000000.0; // ❌ ERROR: field 'balance' is private!

    // Wire Transfer
    let mut foo_acc: BankAccount = BankAccount::new("Foo Bar", AccountType::Savings);
    ops::wire_transfer(&mut gson_acc, &mut foo_acc, 2000.00);
    println!(
        "User: {}, Balance: ${}",
        gson_acc.holder,
        gson_acc.check_balance()
    );
}
