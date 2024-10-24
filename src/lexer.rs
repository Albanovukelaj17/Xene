#[derive(Clone, Debug,PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Var,
    Identifier(String),
    Number(i64),
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Semicolon,
    Print,
    If,
    Else,
    While,
    For,
    Til,
    In,
    Range, // 1..10
    Switch,
    Case,
    Default,
    Colon,
    Break,
    Eof,

}
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();  // Liste der erkannten Tokens
    let chars: Vec<char> = input.chars().collect();  // Wandelt den Eingabetext in eine Zeichenliste um
    let mut i = 0;  // Index für die Schleife

    while i < chars.len() {
        match chars[i] {
            '=' => {
                tokens.push(Token::Equal);

            }
            '+' => {
                tokens.push(Token::Plus);

            }
            '-' => {
                tokens.push(Token::Minus);

            }
            '*' => {
                tokens.push(Token::Multiply);

            }
            '/' => {
                tokens.push(Token::Divide);

            }
            '%' => {
                tokens.push(Token::Modulo);

            }
            '(' => {
                tokens.push(Token::LeftParen);

            }
            ')' => {
                tokens.push(Token::RightParen);
            }
            '{' => {
                tokens.push(Token::LeftBrace);
            }
            '}' => {
                tokens.push(Token::RightBrace);
            }
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
            ';' => {
                tokens.push(Token::Semicolon);
            }
            '.' => {
                // Check for two consecutive dots `..`
                if i + 1 < chars.len() && chars[i + 1] == '.' {
                    tokens.push(Token::Range);
                    i += 1;  // Skip the second dot
                } else {
                    println!("Unbekanntes Zeichen: .");
                }
            }
            ':' =>{tokens.push(Token::Colon);

            }

            ' ' | '\n' => {
                // Ignoriere Leerzeichen und Zeilenumbrüche
            }
            c if c.is_digit(10) => {
                let mut num = String::new();
                while i < chars.len() && chars[i].is_digit(10) {
                    num.push(chars[i]);
                    i += 1;
                }
                i -= 1;
                let number = num.parse::<i64>().unwrap();
                tokens.push(Token::Number(number));
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while i < chars.len() && chars[i].is_alphabetic() {
                    ident.push(chars[i]);
                    i += 1;
                }
                i -= 1;
                match ident.as_str() {
                    "var" => {
                        tokens.push(Token::Var);
                    }
                    "print" => {
                        tokens.push(Token::Print);
                    }
                    "if" => {
                        tokens.push(Token::If);
                    }
                    "else" => {
                        tokens.push(Token::Else);
                    }
                    "while" => {
                        tokens.push(Token::While);
                    }
                    "for" =>{
                        tokens.push(Token::For);
                    }
                    ".." => {
                        tokens.push(Token::Til);
                    }
                    "switch" => tokens.push(Token::Switch),
                    "case" => tokens.push(Token::Case),
                    "default" => tokens.push(Token::Default),
                    "break" => tokens.push(Token::Break),
                    "in"=>{
                        tokens.push(Token::In);
                    }
                    _ => {
                        tokens.push(Token::Identifier(ident.clone()));
                    }
                }
            }
            _ => println!("Unbekanntes Zeichen: {}", chars[i]),
        }
        i += 1;
    }

    tokens.push(Token::Eof);
    tokens
}



