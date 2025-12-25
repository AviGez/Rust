#[derive(Clone, Debug)]
pub struct Transaction {
    from_account: u32,
    to_account: u32,
    amount: i32,
    approved: bool,
}

impl Transaction {
    pub fn new(from_account: u32, to_account: u32, amount: i32) -> Self {
        Self {
            from_account,
            to_account,
            amount,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn print(&self) {
        let status = if self.approved { "APPROVED" } else { "PENDING" };
        println!(
            "Transaction: {} -> {}, Amount: {}, Status: {}",
            self.from_account, self.to_account, self.amount, status
        );
    }

    pub fn is_approved(&self) -> bool {
        self.approved
    }
}
