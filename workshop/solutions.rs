// Library Management System Exercises Solutions

use std::io;
use std::rc::Rc;
use rand::seq::SliceRandom;

// Provided macro for creating custom types.
// - Defines a struct with the name of the custom type ($name), encapsulating another type ($ty).
// - Implements a constructor for the struct, accepting a value convertible into the encapsulated type ($ty).
macro_rules! impl_type {
    ($name:ident, $ty:ty) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct $name($ty);

        impl $name {
            pub fn new(value: impl Into<$ty>) -> $name {
                $name(value.into())
            }
        }
    };
}

// Provided calls, creating custom types for the fields of Book.
impl_type!(Title, String); //Title is String
impl_type!(Author, String); //Author is String
impl_type!(ISBN, String); //ISBN is String
impl_type!(PublicationYear, u16); //PublicationYear is u16

// Provided implementation of 'len' for Title.
impl Title {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn main() {
    //TODO add to exercise: "Create a vec of books"
    let mut books:Vec<Book> = vec![];
    
    // From Exercise 1.1: Structs & Option Struct
    let book1 = Book {
        title: Title::new("1984"),
        author: Author::new("George Orwell"),
        isbn: ISBN::new("9780451524935"),
        publication_year: Some(PublicationYear(1961)),
    };
    let book2 = Book {
        title: Title::new("Brave New World"),
        author: Author::new("Aldous Huxley"),
        isbn: ISBN::new("9780060850524"),
        publication_year: Some(PublicationYear(2006)),
    };
    println!("{:?}", book1);
    println!("{:?}", book2);

    // From Exercise 4.3: Crates
    suggest_book(&books);

    // From Exercise 6.2: Error Handling
    match search_book(&mut books, ISBN::new("9780060850524")) {
        Ok(book) => { println!("Found book: {:?}", book); }
        Err(e) => { println!("Error: {:?}", e); }
    }

    // From Exercise 3.3: Enums & Pattern matching
    user_input(&mut books);
}

// For Exercise 3.3: Enums & Pattern Matching
// A provided function handling command line input for interaction with the program.
// Takes a mutable Vector of Book as a parameter.
// Uses LibraryAction enum and handle_action function from Exercise 4 to execute functionality.
pub fn user_input(books: &mut Vec<Book>) {
    let mut is_first_interaction = true;

    loop {
        if !is_first_interaction {
            println!("Press Enter to continue.:");
            io::stdin().read_line(&mut String::new()).expect("Failed to read input");
        } else {
            is_first_interaction = false;
        }

        println!("Enter your selection:");
        println!("1) Add a Book");
        println!("2) Take a Book");
        println!("3) List all Books");
        println!("0) Quit");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        let user_input = user_input.trim();

        let action = match user_input {
            "0" => break,
            "1" => {
                println!("Enter Book details (title, author, ISBN, year):");
                let mut details = String::new();
                io::stdin().read_line(&mut details).expect("Failed to read input");
                let parts: Vec<&str> = details.trim().split(',').collect();
                if parts.len() < 3 {
                    println!("\nInvalid input! \"title, author, ISBN, year\" required.");
                    continue;
                }

                let year = match parts.get(3) {
                    Some(year_str) => match year_str.trim().parse::<u16>() {
                        Ok(year) => Some(year),
                        Err(_) => {
                            println!("\nInvalid input! Year must be u16.");
                            continue;
                        }
                    },
                    None => None,
                };

                LibraryAction::AddBook(
                    Title::new(parts[0].trim().to_string()),
                    Author::new(parts[1].trim().to_string()),
                    ISBN::new(parts[2].trim().to_string()),
                    year.map(|it| PublicationYear::new(it)),
                )
            }
            "2" => {
                println!("Enter ISBN of the book to take:");
                let mut isbn = String::new();
                io::stdin().read_line(&mut isbn).expect("Failed to read input");
                LibraryAction::TakeBook(ISBN::new(isbn.trim()))
            }
            "3" => LibraryAction::ListBooks,
            _ => {
                println!("\nInvalid choice!");
                continue;
            }
        };

        handle_action(books, action);
    }
}

// 1. Structs & Option Struct
// 1.1 Create a 'Book' struct with fields for the title, author, ISBN and an optional publication year.
//     To achieve better type safety thorugh enforcing stricter type checks, use the provided Types Title, Author, ISBN and PublicationYear defined at the beginning of the file.
//     Hint: If you look at the impl_type(name, type) macro calls, you can see the wrapped types of each of the provided types.
// 1.2 Create instances of 'Book' and print their details.
#[derive(Debug)]
struct Book {
    title: Title,
    author: Author,
    isbn: ISBN,
    publication_year: Option<PublicationYear>,
}

// 2. Ownership & Borrow Checker TODO
// 2.1 Write a function add_book, that takes ownership of a book and adds it to a borrowed book collection.
// 2.2 Write a function take_book, with which you can take a book out of a borrowed book collection by its ISBN.
//     The function takes ownership of a book from the collection and returns it.
// 2.3 Write a function list_books, that borrows a book collection and prints it.
fn add_book(books: &mut Vec<Book>, book: Book) {
    books.push(book);
}

fn take_book(books: &mut Vec<Book>, isbn: &ISBN) -> Option<Book> {
    let index = books.iter().position(|book| &book.isbn == isbn); //maybe provide
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

// 3. Enums & Pattern Matching
// 3.1 Create an enum 'LibraryAction' with actions 'AddBook(Title, Author, ISBN, Option<PublicationYear>)', 'TakeBook(ISBN)' and 'ListBooks'.
// 3.2 Create a function 'handle_action'.
//     Use a match statement to handle each LibraryAction, calling the methods you wrote in Task 3.
// 3.3 Uncomment the 'user_input' function and its call in main()
//     The function 'user_input' sets a variable to an action the user chooses and calls the function 'handle_action' with the chosen action.
enum LibraryAction {
    AddBook(Title, Author, ISBN, Option<PublicationYear>),
    TakeBook(ISBN),
    ListBooks,
}

fn handle_action(books: &mut Vec<Book>, action: LibraryAction) {
    match action {
        LibraryAction::AddBook(title, author, isbn, publication_year) => {
            let title_clone = title.clone();
            let book = Book {
                title,
                author,
                isbn,
                publication_year
            };
            add_book(books, book);
            println!("Added Book: {:?}", title_clone);
        },
        LibraryAction::TakeBook(isbn) => {
            let removed_book = take_book(books, &isbn);
            println!("Took Book: {:?}", removed_book);
        },
        LibraryAction::ListBooks => {
            list_books(books);
        },
    }
}

// 4. Crates
// 4.1 Add to dependencies in Cargo.toml: rand = "0.8.5"
//     and import rand::seq::SliceRandom
// 4.2 Write a function 'suggest_book', that chooses a random book using 'books.choose(&mut rand::thread_rng())' and prints the result.
fn suggest_book(books: &Vec<Book>) {
    if let Some(book) = books.choose(&mut rand::thread_rng()) {
        println!("Suggested Book: {:?}", book);
    } else {
        println!("No books available to suggest.");
    }
}

// 5. Lifetimes
// Create a function, which takes two references to Book as parameters and returns a reference to the book with the longest title.
// TODO further explanation...
fn longest_title<'a>(x: &'a Book, y: &'a Book) -> &'a Book {
    if x.title.len() > y.title.len() {
        x
    } else {
        y
    }
}

// 6. Error Handling
// 6.1 Write a function that searches for a book by its isbn, but doesn't take ownership of it.
//     Return a Result of a Book or an error message, depending on if the book was found.
// 6.2 Call the function and use pattern matching to handle the possible results.
fn search_book(books: &mut Vec<Book>, isbn: ISBN) -> Result<&Book, String> {
    //panic!("The Library burned down");
    books.iter().find(|&book| book.isbn == isbn).ok_or_else(|| "Book not found".into())
}

// 7. Smart Pointers
// 7.1 Box
// 7.2 Rc


// (Macros)
// Write a macro to log library actions, use it to log actions.
// macro_rules! log_action {
//     ($msg:expr) => {
//         println!("LOG: {}", $msg)
//     };
// }
