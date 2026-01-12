// use super::account::BankAccount;  // Using 'super' to access one level up
use crate::banking::account::BankAccount; // Using 'crate' to access from the module (bank/) level

pub fn wire_transfer(from: &mut BankAccount, to: &mut BankAccount, amount: f64) {
    if from.check_balance() >= amount {
        from.withdraw(amount);
        println!(
            "Transferring ${} from {} to {}...",
            amount, from.holder, to.holder
        );
    } else {
        println!("Transfer failed: Insufficient funds.");
    }
}
