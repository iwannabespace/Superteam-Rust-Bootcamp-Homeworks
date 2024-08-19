pub trait Account {
    fn deposit(&mut self, amount: u64) -> u64;
    fn withdraw(&mut self, amount: u64) -> Result<u64, String>;
    fn balance(&self) -> u64;
}

pub struct BankAccount {
    pub account_number: u32,
    pub holder_name: String,
    // Balance is not floating point
    // because i don't wanna deal with floating point comparisons
    pub balance: u64 
}

impl BankAccount {
    pub fn create(an: u32, holder: &str, b: u64) -> Self {
        Self {
            account_number: an,
            holder_name: String::from(holder),
            balance: b
        }
    }
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: u64) -> u64 {
        self.balance += amount;
        self.balance
    }
    
    // Returns the remaining balance as a Result type 
    // because withdraw might fail depending on the account balance 
    fn withdraw(&mut self, amount: u64) -> Result<u64, String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(self.balance)
        } else {
            Err(format!("Balance is not enough for withdrawing {}", amount))
        }
    }

    fn balance(&self) -> u64 {
        self.balance
    }
}
