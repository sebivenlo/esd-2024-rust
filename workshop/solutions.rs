// Library Management System Exercises Solutions

pub fn main() {
    // From Exercise: Structs & Option Struct
    let book1 = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
        isbn: String::from("9780451524935"),
        publication_year: Some(1961),
    };
    let book2 = Book {
        title: String::from("Brave New World"),
        author: String::from("Aldous Huxley"),
        isbn: String::from("9780060850524"),
        publication_year: Some(2006),
    };
    println!("{:?}", book1);
    println!("{:?}", book2);
}

// 1. Structs & Option Struct
// 1.1 Create a 'Book' struct with fields for the title, author, ISBN and an optional publication year.
// 1.2 Create instances of 'Book' and print their details.
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    publication_year: Option<u32>,
}

// 2. Immutability & Everything non-nullable by default

// 3. Ownership & Borrow Checker
// 3.1 Write a function add_book, that takes ownership of a book and adds it to a borrowed book collection.
// 3.2 Write a function take_book, with which you can take a book out of a borrowed book collection by its ISBN.
//     The function takes ownership of a book from the collection and returns it.
// 3.3 Write a function list_books, that borrows a book collection and prints it.
fn add_book(books: &mut Vec<Book>, book: Book) {
    books.push(book);
}

fn take_book(books: &mut Vec<Book>, isbn: String) -> Option<Book> {
    let index = books.iter().position(|book| book.isbn == isbn);
    if let Some(idx) = index {
        let removed_book = books.remove(idx);
        Some(removed_book)
    } else {
        None
    }
}

fn list_books(books: &Vec<Book>) {
    println!("Books: {:?}", books);
}

// 4. Enums & Pattern Matching
// Create an enum 'LibraryAction' with actions 'AddBook', 'TakeBook' and 'ListBooks'.
// Use a match statement to handle each action in the library management system.
enum LibraryAction {
    AddBook(Book),
    TakeBook(String),
    ListBooks,
}

fn handle_action(action: LibraryAction) {
    match action {
        LibraryAction::AddBook(book) => add_book(book),
        LibraryAction::RemoveBook(isbn) => remove_book(isbn),
        LibraryAction::ListBooks => list_books(),
    }
}

// Macros

// Lifetimes

// Box

// (Smart Pointer)

// (Error Handling)

// (Generics=Traits)

// ((Async Rust))
