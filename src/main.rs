mod lexer;  // Importiere deine lexer.rs Datei
mod parser;

use std::io::{self, Write};  // Zum Verarbeiten der Eingabe/Ausgabe
use lexer::tokenize;  // Importiere die tokenize-Funktion aus deinem Lexer
use parser::parse_assignment;


fn main() {
    println!("Willkommen bei Xene!");

    // Starte eine REPL (Read-Eval-Print-Loop)
    loop {
        print!("xene> "); // Zeige den Eingabeaufforderung an
        io::stdout().flush().unwrap(); // Leere den Puffer, damit der Prompt sofort angezeigt wird

        let mut input = String::new(); // Erstelle einen neuen String für die Benutzereingabe
        io::stdin().read_line(&mut input).expect("Fehler beim Lesen der Eingabe");

        let trimmed = input.trim(); // Entferne überflüssige Leerzeichen und Zeilenumbrüche

        if trimmed == "exit" {
            println!("Auf Wiedersehen!");
            break; // Beende die Schleife, wenn der Benutzer 'exit' eingibt
        }

        // Tokenisiere die Benutzereingabe und gib die Tokens aus
        let tokens = tokenize(trimmed);
        for token in tokens {
            println!("{:?}", token); // Gib jedes Token aus
        }
    }
}
