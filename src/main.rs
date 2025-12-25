mod customer;
mod account;
mod transaction;
mod bank;

use customer::Customer;
use account::Account;
use bank::Bank;

fn main() {
    println!("===== BANKING SYSTEM DEMO =====");

    // 1. Create 2 different customers
    println!("\n[Step 1] Creating customers...");
    let customer1 = Customer::new(1, "Alice".to_string(), 30);
    let customer2 = Customer::new(2, "Bob".to_string(), 25);
    customer1.print();
    customer2.print();

    // 2. Create 2 accounts with different owners
    println!("\n[Step 2] Creating accounts...");
    let account1 = Account::new(101, 1000, customer1.clone());
    let account2 = Account::new(102, 500, customer2.clone());
    account1.print();
    account2.print();

    // 3. Create bank and add accounts
    println!("\n[Step 3] Creating bank and adding accounts...");
    let mut bank = Bank::new("MyBank".to_string());
    bank.add_account(account1);
    bank.add_account(account2);
    bank.print();

    // 4. Perform a valid transfer (small amount)
    println!("[Step 4] Performing valid transfer (100 from 101 to 102)...");
    bank.transfer(101, 102, 100);
    bank.print();

    // 5. Perform an invalid transfer (too much money)
    println!("[Step 5] Attempting invalid transfer (600 from 102 to 101)...");
    bank.transfer(102, 101, 600);
    bank.print();

    // 6. Change owner of account
    println!("[Step 6] Changing owner of account 101 to Bob...");
    let new_owner = Customer::new(2, "Bob".to_string(), 25);
    if bank.change_account_owner(101, new_owner) {
        println!("Ownership change successful!");
    } else {
        println!("Ownership change failed!");
    }
    bank.print();

    println!("\n===== END OF DEMO =====");
}
