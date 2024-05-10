// Rust has a set of predefined items in the standard library.
// It brings these items into the scope of every program.
// This set is called a `prelude`.
// If a type is not in the prelude, it has to be brought in via the 'use' statement.

use rand::Rng;
use std::cmp::Ordering;
use std::io; // Input/output library is brought into scope.

// `fn` declares a new function
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // 'let' creates a variable to store user input
        // Variables are immutable by default in Rust.
        // Every value is essentially a const/constexpr
        // `mut` makes a variable mutable`

        let mut guess = String::new(); // This variable binds to the return value of String::new()

        // :: means that new is an associative function of String
        // Create a mutable value that is bound to a new instance of a String.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// Processing a Guess
// 1. Take input /

// Binary crate = executable
// Library crate = dependency
