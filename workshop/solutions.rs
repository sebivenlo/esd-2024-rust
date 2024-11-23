// Library Management System Exercises Solutions

use rand::seq::SliceRandom;
use std::io;
use std::rc::Rc;

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

// Provided implementation of 'len' function for Title.
impl Title {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn main() {
    // From Exercise 1.4: Structs & Option Struct
    let mut library = Library { books: Vec::new() };

    // From Exercise 1.2: Structs & Option Struct
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
    let book3 = Book {
        title: Title::new("Book3"),
        author: Author::new("Author3"),
        isbn: ISBN::new("9876543210987"),
        publication_year: None,
    };
    //println!("{:?}", book1);
    //println!("{:?}", book2);
    //println!("{:?}", book3);

    // From Exercise 3: Ownership
    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);
    //println!("{:?}", book1);
    //println!("{:?}", book2);
    //println!("{:?}", book3);

    // From Exercise 6: Lifetimes & Borrow Checker
    lifetime_demo(&mut library);

    // From Exercise 8: Smart Pointers
    let book1 = Book {
        title: Title::new("1984"),
        author: Author::new("George Orwell"),
        isbn: ISBN::new("9780451524935"),
        publication_year: Some(PublicationYear(1961)),
    };
    library.add_book(book1);
    assign_multiple_owners_to_book(&mut library, &ISBN::new("9780451524935"));

    // From Exercise 4.3: Enums & Pattern matching
    user_input(&mut library);
}

// For Exercise 4.3: Enums & Pattern Matching
// A provided function handling command line input for interaction with the program.
// Takes a mutable Library as a parameter.
// Uses LibraryAction enum and handle_action function from Exercise 4 to execute functionality.
fn user_input(library: &mut Library) {
    let mut is_first_interaction = true;

    loop {
        if !is_first_interaction {
            println!("Press Enter to continue.:");
            io::stdin()
                .read_line(&mut String::new())
                .expect("Failed to read input");
        } else {
            is_first_interaction = false;
        }

        println!("Enter your selection:");
        println!("1) Add a Book");
        println!("2) Take a Book");
        println!("3) List all Books");
        println!("4) Suggest a Book");
        println!("5) Search a Book");
        println!("0) Quit");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        let user_input = user_input.trim();

        let action = match user_input {
            "0" => break,
            "1" => {
                println!("Enter Book details (title, author, ISBN, year):");
                let mut details = String::new();
                io::stdin()
                    .read_line(&mut details)
                    .expect("Failed to read input");
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
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read input");
                LibraryAction::TakeBook(ISBN::new(isbn.trim()))
            }
            "3" => LibraryAction::ListBooks,
            "4" => LibraryAction::SuggestBook,
            "5" => {
                println!("Enter ISBN of the book to search:");
                let mut isbn = String::new();
                io::stdin()
                    .read_line(&mut isbn)
                    .expect("Failed to read input");
                LibraryAction::SearchBook(ISBN::new(isbn.trim()))
            }
            _ => {
                println!("\nInvalid choice!");
                continue;
            }
        };

        handle_action(library, action);
    }
}

// 1. Structs & Option Struct
// 1.1 Create a 'Book' struct with fields for the title, author, ISBN and an optional publication year.
//     To achieve better type safety through enforcing stricter type checks, use the provided Types 'Title', 'Author', 'ISBN' and 'PublicationYear' defined in this File.
//     Hint: If you look at the impl_type(name, type) macro calls (Line 23+), you can see the wrapped types of each of the provided types.
// 1.2 Create instances of 'Book' in the main function and print their details.
//     To initialize the provided types, use the formats:
//     - Title::new("1984") or Title(String::from("1984"))
//     - For PublicationYear use PublicationYear::new(2001_u16) or PublicationYear(2001)
// 1.3 Create a 'Library' struct with a field 'books' of Type 'Vec<Book>'.
// 1.4 Initialize the Library struct and assign it to a variable in the main function.

#[derive(Debug)]
struct Book {
    title: Title,
    author: Author,
    isbn: ISBN,
    publication_year: Option<PublicationYear>,
}

struct Library {
    books: Vec<Book>,
}

// 2. Adding functions to a struct
// 2.1 Implement a function 'add_book' for the Library struct, that takes ownership of a book and adds it to the book collection.
// 2.2 Implement a function 'take_book' for the Library struct, with which you can take a book out of the book collection by its ISBN.
//     The function takes ownership of a book from the collection and returns it or 'None', depending on if the book is found.
//     Hint: use 'self.books.iter().position(|book| &book.isbn == isbn)' to get the index of the searched book.
// 2.3 Implement a function 'get_books' for the Library struct, that returns an immutable book collection from the Library.

impl Library {
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn take_book(&mut self, isbn: &ISBN) -> Option<Book> {
        let index = self.books.iter().position(|book| &book.isbn == isbn);
        if let Some(idx) = index {
            let removed_book = self.books.remove(idx);
            Some(removed_book)
        } else {
            None
        }
    }

    fn get_books(&self) -> &Vec<Book> {
        &self.books
    }
}

// 3. Ownership
// 3.1 In the main function, add the two instances of Book implemented in task 1.2 to the Library.
// 3.2 Try to print the books after adding them to the Library. What happens?
//     -> Ownership of the Book instances has been moved to the book vector.

// 4. Enums & Pattern Matching
// In this task you will create an enum and a function to handle some of the command line functionality.
// User input itself is handled and provided in the 'user_input' function, which calls a function 'handle_action'.
// 4.1 Create an enum 'LibraryAction' with actions:
//     'AddBook(Title, Author, ISBN, Option<PublicationYear>)',
//     'TakeBook(ISBN)',
//     'ListBooks',
//     'SuggestBook',
//     'SearchBook(ISBN)'.
// 4.2 Create a function 'handle_action' and implement the actions 'AddBook', 'TakeBook' and 'SearchBook'.
//     Use a match statement to handle each LibraryAction and call the methods implemented in Task 3.
//     After each action, to give feedback to the CLI, print what action has been done, as well as the book/books.
//     Hint: Also print the book AFTER moving ownership to the book collection. Title/String is a Clone Type.
//     For SuggestBook and SearchBook, uncomment the code and add the enum actions.
// 4.3 Uncomment the 'user_input' function and its call in main()
//     The function 'user_input' sets a variable to an action the user chooses via the CLI and calls the function 'handle_action' with the chosen action.

enum LibraryAction {
    AddBook(Title, Author, ISBN, Option<PublicationYear>),
    TakeBook(ISBN),
    ListBooks,
    // From Exercise 5: Crates
    SuggestBook,
    // From Exercise 7: Error Handling
    SearchBook(ISBN),
}

fn handle_action(library: &mut Library, action: LibraryAction) {
    match action {
        LibraryAction::AddBook(title, author, isbn, publication_year) => {
            let title_clone = title.clone();
            let book = Book {
                title,
                author,
                isbn,
                publication_year,
            };
            library.add_book(book);
            println!("Added Book: {:?}", title_clone);
        }
        LibraryAction::TakeBook(isbn) => {
            let removed_book = library.take_book(&isbn);
            println!("Took Book: {:?}", removed_book);
        }
        LibraryAction::ListBooks => {
            let books = library.get_books();
            println!("Books: {:?}", books);
        }
        // From Exercise 5: Crates
        LibraryAction::SuggestBook => {
            let suggested_book = library.suggest_book();
            if let Some(suggested_book) = suggested_book {
                println!("Suggested Book: {:?}", suggested_book);
            } else {
                println!("No books available to suggest.");
            }
        }
        // From Exercise 7.2: Error Handling
        LibraryAction::SearchBook(isbn) => match library.search_book(&isbn) {
            Ok(book) => {
                println!("Found book: {:?}", book);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        },
    }
}

// 5. Crates
// 5.1 Add to dependencies in Cargo.toml: rand = "0.8.5"
//     In this file, import: rand::seq::SliceRandom
// 5.2 Implement a function 'suggest_book' for the Library struct,
//     that chooses a random book and returns a reference to it.
//     Hint: use the function 'choose(&mut rand::thread_rng())', now provided by the rand crate on the books.
// 5.3 Implement the SuggestBook action in 'handle_action', printing either the suggested book to the CLI,
//     or if the Library is empty, that there are no books available to suggest.

impl Library {
    fn suggest_book(&self) -> Option<&Book> {
        self.books.choose(&mut rand::thread_rng())
    }
}

// 6. Lifetimes & Borrow Checker
// 6.1 Write a function, which takes two references to Book as parameters and returns a reference to the book with the longer title.
// 6.2 Implement function 'lifetime_demo':
//     1. Move a book out of the library into a new variable. Declare a variable that will store the book with the longer title later on.
//     2. Open a new Scope and create a new instance of Book and compare it with the book taken out of the library before the scope.
//        Print the Book with the longer title inside the scope.
//     3. After the Scope, try to first print the book with the longer title and after that the book taken out of the library.
//     4. Run the Program, what happens?

fn longest_title<'a>(x: &'a Book, y: &'a Book) -> &'a Book {
    if x.title.len() > y.title.len() {
        x
    } else {
        y
    }
}

fn lifetime_demo(library: &mut Library) {
    let taken_book = library.take_book(&ISBN::new("9780451524935")).unwrap();
    let longest_title_book;
    {
        let book3 = Book {
            title: Title::new("Book3"),
            author: Author::new("Author3"),
            isbn: ISBN::new("ISB3"),
            publication_year: Some(PublicationYear(3333)),
        };
        longest_title_book = longest_title(&taken_book, &book3);
        println!("Book with longer title: {:?}", longest_title_book);
    }
    //println!("{:?}", longest_title_book);
    println!("Taken Book: {:?}", taken_book);
}

// 7. Error Handling
// 7.1 Implement a function in Library, that searches for a book by its ISBN (Use a reference for good practice),
//     but doesn't take ownership of the book, leaving it in the Library.
//     Return a Result with a Book or an error message, depending on if the book was found.
// 7.2 Implement the SearchBook action in 'handle_action'.
//     Use pattern matching to handle the possible return values.
//     Print the result.

impl Library {
    fn search_book(&self, isbn: &ISBN) -> Result<&Book, &'static str> {
        //panic!("The Library burned down");
        self.books
            .iter()
            .find(|&book| &book.isbn == isbn)
            .ok_or("Book not found")
    }
}

// 8. Smart Pointers
// In this Task, you will create a function, in which multiple Ownership of a book is taken using the Smart Pointer Rc.
// Write a function that borrows a Library and an ISBN.
// - Inside the function, create a new vec 'owners' which takes the type 'Rc<Book>' and initialize it.
// - Take ownership of a book inside the Library by the borrowed ISBN.
// - Create a loop in which you create some amount of owners of the book using Rc and push the owners to the vector.
// - Print how many owners each owned book has.

fn assign_multiple_owners_to_book(library: &mut Library, isbn: &ISBN) {
    let mut owners: Vec<Rc<Book>> = Vec::new();

    let taken_book = Rc::new(library.take_book(isbn).unwrap()); //assume isbn is correct
    let owner = Rc::clone(&taken_book);
    for _ in 0..23 {
        owners.push(owner.clone());
    }

    println!(
        "{:?} has {:?} owners.",
        &taken_book,
        Rc::strong_count(&taken_book)
    );
}

// (Macros)
// Write a macro to log library actions, use it to log actions.
// macro_rules! log_action {
//     ($msg:expr) => {
//         println!("LOG: {}", $msg)
//     };
// }
