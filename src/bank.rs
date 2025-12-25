use crate::account::Account;
use crate::transaction::Transaction;

pub struct Bank {
    name: String,
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

impl Bank {
    pub fn new(name: String) -> Self {
        Self {
            name,
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn print(&self) {
        println!("\n===== BANK: {} =====", self.name);
        println!("\n--- Accounts ---");
        for i in 0..self.accounts.len() {
            self.accounts[i].print();
        }

        println!("\n--- Transactions ---");
        if self.transactions.len() == 0 {
            println!("No transactions");
        } else {
            for i in 0..self.transactions.len() {
                self.transactions[i].print();
            }
        }
        println!();
    }

    pub fn find_account_index(&self, number: u32) -> i32 {
        for i in 0..self.accounts.len() {
            if self.accounts[i].number() == number {
                return i as i32;
            }
        }
        -1
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: i32) -> bool {
        let from_idx = self.find_account_index(from);
        let to_idx = self.find_account_index(to);

        if from_idx < 0 || to_idx < 0 {
            let tx = Transaction::new(from, to, amount);
            self.transactions.push(tx);
            println!("Transfer failed: accounts not found");
            return false;
        }

        let from_idx = from_idx as usize;
        let to_idx = to_idx as usize;

        if self.accounts[from_idx].balance() < amount {
            let tx = Transaction::new(from, to, amount);
            self.transactions.push(tx);
            println!("Transfer failed: insufficient balance");
            return false;
        }

        self.accounts[from_idx].withdraw(amount);
        self.accounts[to_idx].deposit(amount);

        let mut tx = Transaction::new(from, to, amount);
        tx.approve();
        self.transactions.push(tx);

        println!("Transfer successful: {} -> {}, Amount: {}", from, to, amount);
        true
    }

    pub fn change_account_owner(&mut self, account_number: u32, new_owner: crate::customer::Customer) -> bool {
        let idx = self.find_account_index(account_number);
        if idx < 0 {
            return false;
        }
        self.accounts[idx as usize].change_owner(new_owner);
        true
    }
}
