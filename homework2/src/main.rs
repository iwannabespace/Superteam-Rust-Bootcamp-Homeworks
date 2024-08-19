mod account;

use account::{Account, BankAccount};

fn main() {
    let mut acc1 = BankAccount::create(0, "Ertuğrul", 0);
    let mut acc2 = BankAccount::create(1, "Alperen", 150);

    let _ = acc1.deposit(1000);

    if let Err(err) = acc2.withdraw(100) {
        println!("{}", err);
    }

    let acc1_balance = acc1.balance();
    let acc2_balance = acc2.balance();

    println!("Balance of account number: {} is {}", acc1.account_number, acc1_balance);
    println!("Balance of account number: {} is {}", acc2.account_number, acc2_balance);
}
