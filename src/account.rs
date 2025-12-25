use crate::customer::Customer;

pub struct Account {
    number: u32,
    balance: i32,
    owner: Customer,
}

impl Account {
    pub fn new(number: u32, balance: i32, owner: Customer) -> Self {
        Self {
            number,
            balance,
            owner,
        }
    }

    pub fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: i32) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    pub fn change_owner(&mut self, new_owner: Customer) {
        self.owner = new_owner;
    }

    pub fn print(&self) {
        println!(
            "Account Number: {}, Balance: {}, Owner: {}",
            self.number, self.balance, self.owner.name()
        );
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn balance(&self) -> i32 {
        self.balance
    }

    pub fn owner(&self) -> &Customer {
        &self.owner
    }
}
