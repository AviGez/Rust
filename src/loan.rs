#[derive(Clone, Debug)]
pub struct Loan {
    member_id: u32,
    isbn: u32,
    days: u32,
    approved: bool,
}

impl Loan {
    pub fn new(member_id: u32, isbn: u32, days: u32) -> Self {
        Self {
            member_id,
            isbn,
            days,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn print(&self) {
        let status = if self.approved { "APPROVED" } else { "PENDING" };
        println!(
            "Loan - Member: {}, ISBN: {}, Days: {}, Status: {}",
            self.member_id, self.isbn, self.days, status
        );
    }

    pub fn member_id(&self) -> u32 {
        self.member_id
    }

    pub fn isbn(&self) -> u32 {
        self.isbn
    }

    pub fn is_approved(&self) -> bool {
        self.approved
    }
}
