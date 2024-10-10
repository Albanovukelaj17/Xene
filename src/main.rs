mod lexer;
mod parser;
mod interpreter;

use std::collections::HashMap;
use std::io::{self, Write};
use parser::{ASTNode, parse_assignment, parse_expression,parse_if,parse_block};  // Importiere die Parser-Funktionen
use lexer::tokenize;
use interpreter::interpret;
use crate::parser::parse_while;

fn main() {
    println!("Willkommen bei Xene!");

    let mut env = HashMap::new();  // Die Umgebung für Variablen

    loop {
        print!("xene> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Fehler beim Lesen der Eingabe");

        let trimmed = input.trim();

        if trimmed == "exit" {
            println!("Goodbye!");
            break;
        }

        let mut tokens = tokenize(trimmed);

        if let Some(ast) = parse_if(&mut tokens) {
            interpret(ast, &mut env);  // Interpretiere die If-Anweisung
        }
        else if let Some(ast)= parse_while(&mut tokens) {
            interpret(ast, &mut env);
        }
        else if let Some(ast) = parse_assignment(&mut tokens) {
            interpret(ast, &mut env);  // Interpretiere den AST und führe die Zuweisung aus
        }
        // Versuche eine Expression zu parsen
        else if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env);  // Interpretiere die Expression (arithmetische Operation)
        } else {
            println!("Invalid Expression!");
        }
    }
}
