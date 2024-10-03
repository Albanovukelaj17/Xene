mod lexer;
mod parser;
mod interpreter;  // Füge den Interpreter hinzu

use std::collections::HashMap;
use std::io::{self, Write};
use parser::{ASTNode, parse_assignment, parse_expression};  // Importiere die Parser-Funktionen
use lexer::tokenize;
use interpreter::interpret;

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
            println!("Auf Wiedersehen!");
            break;
        }

        let tokens = tokenize(trimmed);

        // Versuche zuerst, eine Zuweisung zu parsen
        if let Some(ast) = parse_assignment(&tokens) {
            interpret(ast, &mut env);  // Interpretiere den AST und führe die Zuweisung aus
        }
        // Versuche eine Expression zu parsen
        else if let Some(ast) = parse_expression(&tokens) {
            interpret(ast, &mut env);  // Interpretiere die Expression (arithmetische Operation)
        } else {
            println!("Ungültiger Ausdruck!");
        }
    }
}
