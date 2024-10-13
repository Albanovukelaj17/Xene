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
    Eof,
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
            '{' => {
                println!("Erkannte öffnende Klammer `{{`");
                tokens.push(Token::LeftBrace)
            }  // Neu hinzugefügt

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


mod tests {
    use super::*;

    #[test]
    fn test_tokenize_var_assignment() {
        let input = "var x = 10;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 6);  // var, x, =, 10, ; ,eof
    }

    #[test]
    fn test_tokenize_binary_expression() {
        let input = "x = x - 1;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 7);  // x, =, x, -, 1, ; , eof
    }

    #[test]
    fn test_tokenize_condition() {
        let input = "if x >= 10;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 6);  // if, x, >=, 10, ;, Eof

        assert_eq!(tokens[0], Token::If);
        assert!(matches!(tokens[1], Token::Identifier(_)));
        assert_eq!(tokens[2], Token::GreaterEqual);
        assert!(matches!(tokens[3], Token::Number(10)));
        assert_eq!(tokens[4], Token::Semicolon);
        assert_eq!(tokens[5], Token::Eof);
    }
    #[test]
    fn test_tokenize_multiple_statements() {
        let input = "var x = 5; x = x + 1;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 12);  // var, x, =, 5, ;, x, =, x, +, 1, ;, Eof

        assert_eq!(tokens[0], Token::Var);
        assert!(matches!(tokens[1], Token::Identifier(_)));
        assert_eq!(tokens[2], Token::Equal);
        assert!(matches!(tokens[3], Token::Number(5)));
        assert_eq!(tokens[4], Token::Semicolon);

        assert!(matches!(tokens[5], Token::Identifier(_)));
        assert_eq!(tokens[6], Token::Equal);
        assert!(matches!(tokens[7], Token::Identifier(_)));
        assert_eq!(tokens[8], Token::Plus);
        assert!(matches!(tokens[9], Token::Number(1)));
        assert_eq!(tokens[10], Token::Semicolon);
    }
}

#[test]
fn test_tokenize_with_braces_and_parens() {
    let input = "if (x > 5) { print(x); }";
    let tokens = tokenize(input);
    assert_eq!(tokens.len(), 14);  // if, (, x, >, 5, ), {, print, (, x, ), ;, }

    assert_eq!(tokens[0], Token::If);                      // if
    assert_eq!(tokens[1], Token::LeftParen);               // (
    assert!(matches!(tokens[2], Token::Identifier(_)));     // x
    assert_eq!(tokens[3], Token::GreaterThan);             // >
    assert!(matches!(tokens[4], Token::Number(5)));         // 5
    assert_eq!(tokens[5], Token::RightParen);              // )
    assert_eq!(tokens[6], Token::LeftBrace);               // {
    assert_eq!(tokens[7], Token::Print);                   // print
    assert_eq!(tokens[8], Token::LeftParen);               // (
    assert!(matches!(tokens[9], Token::Identifier(_)));     // x
    assert_eq!(tokens[10], Token::RightParen);             // )
    assert_eq!(tokens[11], Token::Semicolon);               //;
    assert_eq!(tokens[12], Token::RightBrace);             // }
    assert!(matches!(tokens[13], Token::Eof));
}
