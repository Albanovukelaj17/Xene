use crate::lexer::Token;  // Importiere die Tokens aus dem Lexer

#[derive(Debug, Clone)]

pub enum ASTNode {
    Assignment { var_name: String, value: Box<ASTNode> },
    Number(i64),
    Identifier(String),  // Um Variablen wie `x` zu repräsentieren
    BinaryOp { left: Box<ASTNode>, operator: Token, right: Box<ASTNode> },
    Block(Vec<ASTNode>),
    If {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    While {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    Print(Box<ASTNode>),  // Füge dies hinzu, um die `print`-Anweisung zu unterstützen
}


pub fn parse_assignment(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::Var) = tokens.get(0).cloned() {
        tokens.remove(0);  // Entferne `var`
        if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
            tokens.remove(0);  // Entferne den Variablennamen
            if let Some(Token::Equal) = tokens.get(0).cloned() {
                tokens.remove(0);  // Entferne das Gleichheitszeichen `=`
                if let Some(Token::Number(value)) = tokens.get(0).cloned() {
                    tokens.remove(0);  // Entferne die Zahl
                    return Some(ASTNode::Assignment {
                        var_name: var_name.clone(),
                        value: Box::new(ASTNode::Number(value)),
                    });
                }
            }
        }
    }
    None
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    println!("Current token in parse_expression: {:?}", tokens.get(0));

    // Handhabung für `print`
    if let Some(Token::Print) = tokens.get(0) {
        tokens.remove(0);  // Entferne `print`

        // Erwarte die öffnende Klammer `(`
        if let Some(Token::LeftParen) = tokens.get(0) {
            println!("Erkannte öffnende Klammer `(`");
            tokens.remove(0);  // Entferne `(`

            // Erwarte den Ausdruck innerhalb der Klammern (z.B. eine Variable)
            if let Some(expression) = parse_primary_expression(tokens) {
                println!("Current token in parse_expression: {:?}", tokens.get(0));

                // Erwarte die schließende Klammer `)`
                if let Some(Token::RightParen) = tokens.get(0) {
                    println!("Erkannte schließende Klammer `)`");
                    tokens.remove(0);  // Entferne `)`

                    // Jetzt sollte ein Semikolon `;` folgen
                    if let Some(Token::Semicolon) = tokens.get(0) {
                        println!("Erkannte Semikolon `;` nach Ausdruck");
                        tokens.remove(0);  // Entferne das Semikolon
                        return Some(ASTNode::Print(Box::new(expression)));  // Gib die `print`-Anweisung zurück
                    } else {
                        println!("Fehler: Fehlendes Semikolon nach `print`");
                        return None;
                    }
                } else {
                    println!("Fehler: Fehlende schließende Klammer `)` nach Ausdruck");
                    return None;
                }
            } else {
                println!("Fehler: Ungültiger Ausdruck in `print`");
                return None;
            }
        } else {
            println!("Fehler: Fehlende öffnende Klammer `(` nach `print`");
            return None;
        }
    }

    // Falls es keine `print`-Anweisung ist, versuche eine andere Expression
    if let Some(left) = parse_primary_expression(tokens) {
        if let Some(operator) = tokens.get(0).cloned() {
            tokens.remove(0);  // Entferne den Operator
            match operator {
                Token::GreaterThan | Token::LessThan | Token::GreaterEqual | Token::LessEqual => {
                    if let Some(right) = parse_primary_expression(tokens) {
                        return Some(ASTNode::BinaryOp {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        });
                    } else {
                        println!("Fehler: Erwarte rechten Ausdruck nach Operator");
                        return None;
                    }
                }
                _ => {
                    return Some(left);
                }
            }
        }
    }

    None
}

pub fn parse_primary_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    // Prüfe auf Zahlen oder Variablen als primäre Ausdrücke
    if let Some(Token::Number(value)) = tokens.get(0).cloned() {
        tokens.remove(0);
        return Some(ASTNode::Number(value));
    }

    if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
        tokens.remove(0);
        return Some(ASTNode::Identifier(var_name));
    }

    None
}

pub fn parse_if(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::If) = tokens.get(0).cloned() {
        tokens.remove(0);  // Entferne `if`
        let condition = parse_expression(tokens)?;  // Parse die Bedingung
        let then_branch = parse_block(tokens)?;  // Parse den `then`-Block

        // Prüfe, ob es einen `else`-Block gibt
        let else_branch = if let Some(Token::Else) = tokens.get(0) {
            tokens.remove(0);  // Entferne `else`
            Some(Box::new(parse_block(tokens)?))  // Verpacke den else-Block in eine Box
        } else {
            None
        };

        return Some(ASTNode::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,  // Verwende die Box für else_branch
        });
    }
    None
}

//if x > 5 { print(x); } else { print(0); }
pub fn parse_block(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    // Überprüfen, ob der Block mit `{` beginnt
    if let Some(Token::LeftBrace) = tokens.get(0) {
        tokens.remove(0);  // Entferne die `{`

        let mut statements = Vec::new();

        // Parsen der Anweisungen innerhalb des Blocks
        while let Some(token) = tokens.get(0) {
            println!("Current token: {:?}", token);  // Debugging-Ausgabe

            // Wenn wir auf `}` stoßen, wissen wir, dass der Block endet
            if let Token::RightBrace = token {
                tokens.remove(0);  // Entferne die `}`
                return Some(ASTNode::Block(statements));  // Gib den Block zurück
            }

            // Versuche, den nächsten Ausdruck oder die nächste Anweisung zu parsen
            if let Some(statement) = parse_expression(tokens) {
                statements.push(statement);
            } else {
                // Fehler: Ungültige Anweisung innerhalb des Blocks
                println!("Fehler beim Parsen des Blocks");
                return None;
            }
        }
    }

    println!("Fehler: Block beginnt nicht mit `brace`");
    None
}

pub fn parse_binary_op(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    // Erwarte eine Zahl oder eine Variable auf der linken Seite
    if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
        tokens.remove(0);  // Entferne die Variable oder die Zahl

        // Erwarte einen Operator (z.B. `>`, `<`, `==`, `+`, `-`, etc.)
        if let Some(operator) = tokens.get(0).cloned() {
            tokens.remove(0);  // Entferne den Operator

            // Erwarte eine Zahl oder eine Variable auf der rechten Seite
            if let Some(Token::Number(right_val)) = tokens.get(0).cloned() {
                tokens.remove(0);  // Entferne die rechte Seite

                return Some(ASTNode::BinaryOp {
                    left: Box::new(ASTNode::Identifier(var_name.clone())),
                    operator: operator.clone(),
                    right: Box::new(ASTNode::Number(right_val)),
                });
            }
        }
    }
    None
}
