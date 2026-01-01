use crate::member::Member;
use crate::book::Book;
use crate::loan::Loan;

pub struct Library {
    name: String,
    members: Vec<Member>,
    books: Vec<Book>,
    loans: Vec<Loan>,
}

impl Library {
    pub fn new(name: String) -> Self {
        Self {
            name,
            members: Vec::new(),
            books: Vec::new(),
            loans: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member);
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn deactivate_member(&mut self, id: u32) -> bool {
        let idx = self.find_member_index(id);
        if idx < 0 {
            return false;
        }
        self.members[idx as usize].deactivate();
        true
    }

    pub fn print(&self) {
        println!("\n===== LIBRARY: {} =====", self.name);

        println!("\n--- Members ---");
        for i in 0..self.members.len() {
            self.members[i].print();
        }

        println!("\n--- Books ---");
        for i in 0..self.books.len() {
            self.books[i].print();
        }

        println!("\n--- Loans ---");
        if self.loans.len() == 0 {
            println!("No loans");
        } else {
            for i in 0..self.loans.len() {
                self.loans[i].print();
            }
        }
        println!();
    }

    pub fn find_member_index(&self, id: u32) -> i32 {
        for i in 0..self.members.len() {
            if self.members[i].id() == id {
                return i as i32;
            }
        }
        -1
    }

    pub fn find_book_index(&self, isbn: u32) -> i32 {
        for i in 0..self.books.len() {
            if self.books[i].isbn() == isbn {
                return i as i32;
            }
        }
        -1
    }

    pub fn find_loan_index(&self, member_id: u32, isbn: u32) -> i32 {
        for i in 0..self.loans.len() {
            if self.loans[i].member_id() == member_id
                && self.loans[i].isbn() == isbn
                && self.loans[i].is_approved()
            {
                return i as i32;
            }
        }
        -1
    }

    pub fn borrow_book(&mut self, member_id: u32, isbn: u32, days: u32) -> bool {
        let member_idx = self.find_member_index(member_id);
        let book_idx = self.find_book_index(isbn);

        // Check if member exists and is active
        if member_idx < 0 || !self.members[member_idx as usize].is_active() {
            let loan = Loan::new(member_id, isbn, days);
            self.loans.push(loan);
            println!("Borrow failed: member not found or inactive");
            return false;
        }

        // Check if book exists
        if book_idx < 0 {
            let loan = Loan::new(member_id, isbn, days);
            self.loans.push(loan);
            println!("Borrow failed: book not found");
            return false;
        }

        let book_idx = book_idx as usize;

        // Try to borrow a copy (avoid double borrow by checking first)
        if !self.books[book_idx].borrow_copy() {
            let loan = Loan::new(member_id, isbn, days);
            self.loans.push(loan);
            println!("Borrow failed: no copies available");
            return false;
        }

        // Create and approve loan
        let mut loan = Loan::new(member_id, isbn, days);
        loan.approve();
        self.loans.push(loan);

        println!(
            "Borrow successful: Member {} borrowed ISBN {} for {} days",
            member_id, isbn, days
        );
        true
    }

    pub fn return_book(&mut self, member_id: u32, isbn: u32) -> bool {
        let loan_idx = self.find_loan_index(member_id, isbn);
        let book_idx = self.find_book_index(isbn);

        if loan_idx < 0 || book_idx < 0 {
            println!("Return failed: loan or book not found");
            return false;
        }

        let book_idx = book_idx as usize;
        self.books[book_idx].return_copy();

        println!(
            "Return successful: Member {} returned ISBN {}",
            member_id, isbn
        );
        true
    }
}
