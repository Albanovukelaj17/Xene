use crate::lexer::Token;  // Importiere die Tokens aus dem Lexer

#[derive(Debug, Clone)]
pub enum ASTNode {
    Assignment { var_name: String, value: Box<ASTNode> },
    Number(i64),
    Identifier(String),
    BinaryOp { left: Box<ASTNode>, operator: Token, right: Box<ASTNode> },
    Block(Vec<ASTNode>),
    // Neue Knoten für If und While hinzufügen
    If {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    While {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    Print(String)
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
    // Beispiel für eine `print`-Anweisung
    if let Some(Token::Print) = tokens.get(0) {
        tokens.remove(0);  // Entferne das `print`-Token

        // Erwarte eine Variable oder einen Ausdruck nach `print`
        if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
            tokens.remove(0);  // Entferne die Variable

            // Erwarte ein Semikolon nach der Anweisung
            if let Some(Token::Semicolon) = tokens.get(0) {
                tokens.remove(0);  // Entferne das Semikolon
                return Some(ASTNode::Print(var_name));  // Gib die `print`-Anweisung zurück
            }
        }
    }

    // Andere Ausdrücke (wie arithmetische Operationen) parsen
    if let Some(ast) = parse_binary_op(tokens) {
        return Some(ast);
    }

    None
}

pub fn parse_if(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::If) = tokens.get(0).cloned() {
        tokens.remove(0);  // Entferne `if`

        // Parsen der Bedingung
        let condition = parse_expression(tokens)?;  // Parse die Bedingung (wie z.B. x > 5)

        // Parsen des Codeblocks für den `then`-Zweig
        let then_branch = parse_block(tokens)?;  // Parse den Block nach `if`

        // Prüfe, ob es ein `else` gibt
        let else_branch = if let Some(Token::Else) = tokens.get(0) {
            tokens.remove(0);  // Entferne `else`
            Some(Box::new(parse_block(tokens)?))  // Parse den Block nach `else`
        } else {
            None
        };

        return Some(ASTNode::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        });
    }
    None
}


pub fn parse_block(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    // Überprüfen, ob der Block mit `{` beginnt
    if let Some(Token::LeftBrace) = tokens.get(0) {
        tokens.remove(0);  // Entferne die `{`

        let mut statements = Vec::new();

        // Parsen der Anweisungen innerhalb des Blocks
        while let Some(token) = tokens.get(0) {
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

    // Falls kein `{` gefunden wird oder der Block nicht richtig geschlossen wird, gib `None` zurück
    println!("Fehler: Block beginnt nicht mit 'leftbrace oder endet nicht mit rightbrace");
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
