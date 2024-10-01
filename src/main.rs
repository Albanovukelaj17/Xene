mod lexer;

use std::io::{self, Write}; // For handling input/output

fn main() {
    println!("Welcome to Xene!");

    // Starting a REPL (Read-Eval-Print-Loop)
    loop {
        print!("xene> "); // Display a prompt
        io::stdout().flush().unwrap(); // Make sure the prompt is displayed before input

        let mut input = String::new(); // Allocate a new String to store user input
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let trimmed = input.trim(); // Remove any extra whitespace or newlines

        if trimmed == "exit" {
            println!("Goodbye!");
            break; // Exit the loop if the user types 'exit'
        }

        // For now, just echo back the input
        println!("You entered: {}", trimmed);
    }
}
