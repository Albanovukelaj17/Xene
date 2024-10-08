#[derive(Clone, Debug)]
#[allow(dead_code)]
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
    LeftBrace,          // `{` (neu hinzugefügt)
    RightBrace,         // `}` (neu hinzugefügt)
    GreaterThan,        // `>` (neu hinzugefügt)
    LessThan,           // `<` (neu hinzugefügt)
    GreaterEqual,       // `>=` (neu hinzugefügt)
    LessEqual,          // `<=` (neu hinzugefügt)
    Semicolon,           // `;`
    Print,               // Schlüsselwort `print`
    If,                  // Schlüsselwort `if`
    Else,                // Schlüsselwort `else`
    While,               // Schlüsselwort 'while'
    For,                 // Schlüsselwort 'for'
    Switch,              // Schlüsselwort 'switch'
    Eof,                 // Ende des Codes


}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();  // Liste der erkannten Tokens
    let chars: Vec<char> = input.chars().collect();  // Wandelt den Eingabetext in eine Zeichenliste um
    let mut i = 0;  // Index für die Schleife

    while i < chars.len() {
        match chars[i] {
            '=' => tokens.push(Token::Equal),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '%' => tokens.push(Token::Modulo),
            '(' => {
                println!("Erkannte öffnende Klammer `(`");
                tokens.push(Token::LeftParen);
            }
            ')' => {
                println!("Erkannte schließende Klammer `)`");
                tokens.push(Token::RightParen);
            }
            '{' => tokens.push(Token::LeftBrace),  // Neu hinzugefügt
            '}' => tokens.push(Token::RightBrace), // Neu hinzugefügt
            '>' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::GreaterEqual);
                    i += 1;
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '<' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::LessEqual);
                    i += 1;
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            ';' => tokens.push(Token::Semicolon),
            ' ' | '\n' => {},  // Ignoriere Leerzeichen und Zeilenumbrüche
            c if c.is_digit(10) => {
                let mut num = String::new();
                while i < chars.len() && chars[i].is_digit(10) {
                    num.push(chars[i]);
                    i += 1;
                }
                i -= 1;
                tokens.push(Token::Number(num.parse::<i64>().unwrap()));
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while i < chars.len() && chars[i].is_alphabetic() {
                    ident.push(chars[i]);
                    i += 1;
                }
                i -= 1;
                match ident.as_str() {
                    "var" => tokens.push(Token::Var),
                    "print" => tokens.push(Token::Print),
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "while" => tokens.push(Token::While),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            _ => println!("Unbekanntes Zeichen: {}", chars[i]),
        }
        i += 1;
    }

    tokens.push(Token::Eof); // Füge am Ende des Codes ein EOF-Token hinzu
    tokens
}
