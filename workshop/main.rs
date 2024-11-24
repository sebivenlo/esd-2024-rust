// Library Management System Exercises Solutions

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

// For Exercise 4.3: Enums & Pattern Matching
// A provided function handling command line input for interaction with the program.
// Takes a mutable Library as a parameter.
// Uses LibraryAction enum and handle_action function from Exercise 4 to execute functionality.
/*
fn user_input(library: &mut Library) {
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
        println!("4) Suggest a Book");
        println!("5) Search a Book");
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
            "4" => LibraryAction::SuggestBook,
            "5" => {
                println!("Enter ISBN of the book to search:");
                let mut isbn = String::new();
                io::stdin().read_line(&mut isbn).expect("Failed to read input");
                LibraryAction::SearchBook(ISBN::new(isbn.trim()))
            },
            _ => {
                println!("\nInvalid choice!");
                continue;
            }
        };

        handle_action(library, action);
    }
}
*/

pub fn main() {
    // Add your code here
    // Example print statement with debug trait ':?' to print the book struct
    //println!("{:?}", book1);

    // Add all code above the user_input call
    // From Exercise 4.3: Enums & Pattern matching
    //user_input(&mut library);
}

// 1. Structs & Option Struct
// 1.1 Create a 'Book' struct with fields for the title, author, ISBN and an optional publication year.
//     To achieve better type safety through enforcing stricter type checks, use the provided Types 'Title', 'Author', 'ISBN' and 'PublicationYear' defined earlier in this File.
//     Hint: If you look at the impl_type(name, type) macro calls (Line 23+), you can see the wrapped types of each of the provided types.
// 1.2 Create multiple instances of 'Book' in the main function and print their details.
//     To initialize the provided types, use the formats:
//     - For Title, Author & ISBN: Title::new("1984") or Title(String::from("1984"))
//     - For PublicationYear: PublicationYear::new(2001_u16) or PublicationYear(2001)
// 1.3 Create a 'Library' struct with a field 'books' of Type 'Vec<Book>'.
// 1.4 Initialize the Library struct and assign it to a variable in the main function.
//     Hint: Initalize Vec<Book> inside the Library with 'books: Vec::new()'
// Run the program.

#[derive(Debug)] // Used to allow for printing of the book struct for debugging purposes
struct Book {
    // TODO: Implement 1.1
}

// TODO: Implement 1.3


// 2. Adding functions to a struct
// 2.1 Implement a function 'add_book' for the Library struct, that takes ownership of a book and adds it to the book vector.
// 2.2 Implement a function 'take_book' for the Library struct, with which you can take a book out of the book vector by its ISBN.
//     The function takes ownership of a book from the vector and returns it or 'None', depending on if the book is found.
//     Hint: use 'self.books.iter().position(|book| &book.isbn == isbn)' to get the index of the searched book.
//           use the 'remove' method to receive Ownership of a book from the vec.
// 2.3 Implement a function 'get_books' for the Library struct, that returns an immutable book vector from the Library.
//     Note: In Rust, you would normally not write getters and setters, apart from a few edge cases

// TODO: Implement 2.1

// TODO: Implement 2.2

// TODO: Implement 2.3


// 3. Basic Ownership
// 3.1 In the main function, add the instances of Book implemented in task 1.2 to the Library.
// 3.2 Try to print the books after adding them to the Library. What happens?
//     -> Ownership of the Book instances has been moved to the book vector.
//     After running the program, remove/comment the print statements.

// 4. Enums & Pattern Matching
// In this task you will create an enum and a function to handle some of the command line functionality.
// User input itself is handled and provided in the 'user_input' function, which calls a function 'handle_action'.
// 4.1 Create an enum 'LibraryAction' with actions:
//     'AddBook(Title, Author, ISBN, Option<PublicationYear>)',
//     'TakeBook(ISBN)',
//     'ListBooks',
//     'SuggestBook',
//     'SearchBook(ISBN)'.
// 4.2 Uncomment the function 'handle_action' and implement the actions 'AddBook', 'TakeBook' and 'ListBooks'.
//     Use a match statement to handle each LibraryAction and call the methods implemented in Task 3.
//     After each action, to give feedback to the CLI, print what action has been done, as well as the book/books.
//     Hint: Also print the book title AFTER moving ownership to the book vector. Title/String is a Clone Type.
// 4.3 Uncomment the 'user_input' function and its call in main()
//     The function 'user_input' sets a variable to an action the user chooses via the CLI and calls the function 'handle_action' with the chosen action.
// Run the program and test adding a book, taking a book and getting a list of all books.

// TODO: Implement 4.1

/*
fn handle_action(library: &mut Library, action: LibraryAction) {

    /* add match statement */ {
        // TODO: Implement 4.2

        LibraryAction::SuggestBook => {
            println!("Not implemented yet.")
              // TODO Implement 5.3
        }
        LibraryAction::SearchBook(isbn) => {
            println!("Not implemented yet.")
              // TODO Implement 7.2
        },
    }
}
*/


// 5. Working with Crates
// 5.1 Uncomment the suggest_book method implementation underneath.
// 5.2 Add to dependencies in Cargo.toml: rand = "0.8.5"
//     In main.rs, import: rand::seq::SliceRandom
// 5.3 Implement the SuggestBook action in 'handle_action', printing either the suggested book to the CLI,
//     or if the Library is empty, that there are no books available to suggest.
// Run the program and choose suggest book.

/*
impl Library {
    fn suggest_book(&self) -> Option<&Book> {
        self.books.choose(&mut rand::thread_rng())
    }
}
*/


// 6. Lifetimes & Borrow Checker
// 6.1 Write a function, which takes two references to Book as parameters and returns a reference to the book with the longer title.
// 6.2 Implement function 'lifetime_demo':
//     After the Scope, what happens when you try to print the book with the longer title?
// Uncomment what doesn't work and call the function in main() and look at the results.

// TODO: Implement 6.1

// TODO: Implement 6.2
/*
fn lifetime_demo(library: &mut Library) {
    let taken_book = // take a book from the library
    let longest_title_book;
    { // Opening a new Scope
        // Create a new book with a longer title than the one from the library
        longest_title_book = // compare your new book and the one from the library
        println!("Book with longer title: {:?}", longest_title_book);
    }
    println!("{:?}", longest_title_book);
    println!("Taken Book: {:?}", taken_book);
}
    */

// 7. Error Handling
// 7.1 Implement a function in Library, that searches for a book by its ISBN (Use a reference for good practice),
//     but doesn't take ownership of the book, leaving it in the Library.
//     Return a Result with a Book or an error message, depending on if the book was found.
// 7.2 Implement the SearchBook action in 'handle_action'.
//     Use pattern matching to handle the possible return values.
//     Print the result.
// Run the program and search for a book.

//impl Library {
//    /* add function */ {
//        //panic!("The Library burned down");
//        self.books.iter().find(|&book| &book.isbn == isbn)./* call method to return a Result */
//    }
//}

// 8. Smart Pointers
// In this Task, you will create a function, in which multiple Ownership of a book is taken using the Smart Pointer Rc.
// Write a function that borrows a Library and an ISBN.
// - Inside the function, create a new vec 'owners' which takes the type 'Rc<Book>' and initialize it.
// - Take ownership of a book inside the Library by the borrowed ISBN.
// - Create a loop in which you create some amount of owners of the book using Rc and push the owners to the vector.
// - Print how many owners each owned book has.
// Call the function in main and look at the result.

// TODO: Implement 8