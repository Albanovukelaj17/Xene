pub enum Token{
    Var, // Schlüsselwort `le or c´var t`
    Identifier(String),  // Variablenname
    Number(i64),         // Ganzzahl-Zahl
    Float(f64),          // Gleitkommazahl
    Equal,               // Gleichheitszeichen `=`
    Plus,                // Pluszeichen `+`
    Minus,               // Minuszeichen `-`
    Multiply,            // Multiplikationszeichen `*`
    Divide,              // Divisionszeichen `/`
    LeftParen,           // `(`
    RightParen,          // `)`
    Semicolon,           // `;`
    Print,               // Schlüsselwort `print`
    If,                  // Schlüsselwort `if`
    Else,                // Schlüsselwort `else`
    Eof,                 // Ende des Codes
}

pub fn tokenize(input: &str) -> Vec<Token>{
    let mut tokens = Vec::new();  // Liste, die alle erkannten Tokens speichert
    let chars: Vec<char> = input.chars().collect();  // Wandelt den Eingabetext in eine Zeichenliste um
    let mut i = 0;  // Index für die Schleife

    while i < chars.len(){
        match chars[i]{
            '=' => tokens.push(Token::Equal),
        }
    }
}