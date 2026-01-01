#[derive(Clone, Debug)]
pub struct Book {
    isbn: u32,
    title: String,
    copies_total: u32,
    copies_available: u32,
}

impl Book {
    pub fn new(isbn: u32, title: String, copies: u32) -> Self {
        Self {
            isbn,
            title,
            copies_total: copies,
            copies_available: copies,
        }
    }

    pub fn borrow_copy(&mut self) -> bool {
        if self.copies_available > 0 {
            self.copies_available -= 1;
            true
        } else {
            false
        }
    }

    pub fn return_copy(&mut self) {
        if self.copies_available < self.copies_total {
            self.copies_available += 1;
        }
    }

    pub fn print(&self) {
        println!(
            "Book ISBN: {}, Title: {}, Total: {}, Available: {}",
            self.isbn, self.title, self.copies_total, self.copies_available
        );
    }

    pub fn isbn(&self) -> u32 {
        self.isbn
    }

    pub fn copies_available(&self) -> u32 {
        self.copies_available
    }
}
