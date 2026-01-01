mod member;
mod book;
mod loan;
mod library;

use member::Member;
use book::Book;
use library::Library;

fn main() {
    println!("===== LIBRARY MANAGEMENT SYSTEM DEMO =====");

    // 1. Create 2 members
    println!("\n[Step 1] Creating 2 members...");
    let member1 = Member::new(1, "Alice".to_string());
    let member2 = Member::new(2, "Bob".to_string());
    member1.print();
    member2.print();

    // 2. Create 2 books (one with 1 copy, one with 2 copies)
    println!("\n[Step 2] Creating 2 books...");
    println!("  Book 1: ISBN 100 (Rust Programming) - 1 copy");
    println!("  Book 2: ISBN 200 (Data Structures) - 2 copies");
    let book1 = Book::new(100, "Rust Programming".to_string(), 1);
    let book2 = Book::new(200, "Data Structures".to_string(), 2);
    book1.print();
    book2.print();

    // 3. Create library and add members and books
    println!("\n[Step 3] Creating library and adding members/books...");
    let mut library = Library::new("City Library".to_string());
    library.add_member(member1);
    library.add_member(member2);
    library.add_book(book1);
    library.add_book(book2);
    library.print();

    // 4. Valid borrow (active member, book available)
    println!("[Step 4] VALID BORROW: Member 1 (Alice) borrows ISBN 100 for 14 days");
    library.borrow_book(1, 100, 14);
    library.print();

    // 5. Invalid borrow (same book, no copies available)
    println!("[Step 5] INVALID BORROW: Member 2 (Bob) tries to borrow ISBN 100 (no copies)");
    library.borrow_book(2, 100, 14);
    library.print();

    // 6. Valid return
    println!("[Step 6] VALID RETURN: Member 1 (Alice) returns ISBN 100");
    library.return_book(1, 100);
    library.print();

    // 7. Deactivate member and try to borrow (invalid)
    println!("[Step 7] Deactivating Member 1 (Alice)...");
    library.deactivate_member(1);
    println!("Member 1 deactivated!");
    library.print();

    println!("[Step 7b] INVALID BORROW: Member 1 (Alice - now INACTIVE) tries to borrow ISBN 100");
    library.borrow_book(1, 100, 14);
    library.print();

    // 8. Show Member 2 borrowing multiple copies
    println!("[Step 8] VALID BORROW: Member 2 (Bob) borrows ISBN 100 for 14 days");
    library.borrow_book(2, 100, 14);
    library.print();

    println!("[Step 9] VALID BORROW: Member 2 (Bob) borrows ISBN 200 (copy 1) for 21 days");
    library.borrow_book(2, 200, 21);
    library.print();

    println!("[Step 10] VALID BORROW: Member 2 (Bob) borrows ISBN 200 (copy 2) for 21 days");
    library.borrow_book(2, 200, 21);
    library.print();

    println!("[Step 11] INVALID BORROW: Member 2 tries to borrow ISBN 200 again (no copies)");
    library.borrow_book(2, 200, 21);
    library.print();

    println!("\n===== END OF DEMO =====");
}
