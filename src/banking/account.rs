pub enum AccountType {
    Savings,
    Checking,
}

pub struct BankAccount {
    pub holder: String, // Public field (We must explicitly declare it as public)
    pub account_type: AccountType,
    balance: f64, // Private field (Rust's default)
}

impl BankAccount {
    // Public Constructor
    pub fn new(holder: &str, kind: AccountType) -> Self {
        Self {
            holder: holder.to_string(),
            account_type: kind,
            balance: 0.0,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount
    }

    pub fn withdraw(&mut self, amount: f64) {
        match amount <= self.balance {
            true => {
                self.balance -= amount;
                println!("Withdrawal Successful!")
            }
            false => println!(
                "Withdrawal Failed! Insufficient funds to withdraw: ${}",
                amount
            ),
        }
    }

    pub fn check_balance(&self) -> f64 {
        self.balance
    }
}
