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
    Modulo,              // MOdulo %
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
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ';' => tokens.push(Token::Semicolon),
            '%' => tokens.push(Token::Modulo),
            c if c.is_digit(10) => {
                // Eine Zahl erkennen
                let mut num = String::new();
                while i < chars.len() && chars[i].is_digit(10) {
                    num.push(chars[i]);
                    i += 1;
                }
                i -= 1;  // Einen Schritt zurück, da die Schleife einen Schritt zu weit gegangen ist
                tokens.push(Token::Number(num.parse::<i64>().unwrap()));
            }
            c if c.is_alphabetic() => {
                // Einen Variablennamen oder Schlüsselwort erkennen
                let mut ident = String::new();
                while i < chars.len() && chars[i].is_alphabetic() {
                    ident.push(chars[i]);
                    i += 1;
                }
        }
        }
    }
}