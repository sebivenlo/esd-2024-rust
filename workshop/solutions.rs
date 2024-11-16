// Library Management System Exercises Solutions

pub fn main() {}

fn add_book(book: Book) {
    //TODO
}

fn remove_book(isbn: i32) {
    //TODO
}

fn list_books() {
    //TODO
}


// Common Programming Concepts – refresher

// Immutability & Everything non-nullable by default

// Ownership & Borrow Checker

// Macros

// Lifetimes

// Box

// Structs & Option Struct
// Create a 'Book' struct with fields for the title, author, ISBN and an optional publication year.
// Create instances of 'Book' and print their details.
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    isbn: i32,
    publication_year: Option<u32>,
}

// Enums & Pattern Matching
// Create an enum 'LibraryAction' with actions 'AddBook', 'RemoveBook' and 'ListBooks'.
// Use a match statement to handle each action in the library management system.
enum LibraryAction {
    AddBook(Book),
    RemoveBook(isbn),
    ListBooks,
}

fn handle_action(action: LibraryAction) {
    match action {
        LibraryAction::AddBook(book: Book) => add_book(book),
        LibraryAction::RemoveBook(isbn: i32) => remove_book(isbn),
        LibraryAction::ListBooks => list_books(),
    }
}

// (Smart Pointer)

// (Error Handling)

// (Generics=Traits)

// ((Async Rust))
