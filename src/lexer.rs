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
                println!("Erkannter Token: `=` (Equal)");
            }
            '+' => {
                tokens.push(Token::Plus);
                println!("Erkannter Token: `+` (Plus)");
            }
            '-' => {
                tokens.push(Token::Minus);
                println!("Erkannter Token: `-` (Minus)");
            }
            '*' => {
                tokens.push(Token::Multiply);
                println!("Erkannter Token: `*` (Multiply)");
            }
            '/' => {
                tokens.push(Token::Divide);
                println!("Erkannter Token: `/` (Divide)");
            }
            '%' => {
                tokens.push(Token::Modulo);
                println!("Erkannter Token: `%` (Modulo)");
            }
            '(' => {
                tokens.push(Token::LeftParen);
                println!("Erkannter Token: `(` (LeftParen)");
            }
            ')' => {
                tokens.push(Token::RightParen);
                println!("Erkannter Token: `)` (RightParen)");
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                println!("Erkannter Token: `{{` (LeftBrace)");
            }
            '}' => {
                tokens.push(Token::RightBrace);
                println!("Erkannter Token: `}}` (RightBrace)");
            }
            '>' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::GreaterEqual);
                    println!("Erkannter Token: `>=` (GreaterEqual)");
                    i += 1;
                } else {
                    tokens.push(Token::GreaterThan);
                    println!("Erkannter Token: `>` (GreaterThan)");
                }
            }
            '<' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::LessEqual);
                    println!("Erkannter Token: `<=` (LessEqual)");
                    i += 1;
                } else {
                    tokens.push(Token::LessThan);
                    println!("Erkannter Token: `<` (LessThan)");
                }
            }
            ';' => {
                tokens.push(Token::Semicolon);
                println!("Erkannter Token: `;` (Semicolon)");
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
                println!("Erkannter Token: `{}` (Number)", number);
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
                        println!("Erkannter Token: `var` (Var)");
                    }
                    "print" => {
                        tokens.push(Token::Print);
                        println!("Erkannter Token: `print` (Print)");
                    }
                    "if" => {
                        tokens.push(Token::If);
                        println!("Erkannter Token: `if` (If)");
                    }
                    "else" => {
                        tokens.push(Token::Else);
                        println!("Erkannter Token: `else` (Else)");
                    }
                    "while" => {
                        tokens.push(Token::While);
                        println!("Erkannter Token: `while` (While)");
                    }
                    "for" =>{
                        tokens.push(Token::For);
                        println!("Erkannter Token: `for` (For)");
                    }
                    ".." => {
                        tokens.push(Token::Til);
                        println!("Erkannter Token: `..` (Til)");
                    }
                    "in"=>{
                        tokens.push(Token::In);
                        println!("Erkannter Token: `in` (In)");
                    }
                    _ => {
                        tokens.push(Token::Identifier(ident.clone()));
                        println!("Erkannter Token: `{}` (Identifier)", ident);
                    }
                }
            }
            _ => println!("Unbekanntes Zeichen: {}", chars[i]),
        }
        i += 1;
    }

    tokens.push(Token::Eof);
    println!("Erkannter Token: EOF (End of File)");
    tokens
}



