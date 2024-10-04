use crate::lexer::Token;  // Importiere die Tokens aus dem Lexer

#[derive(Debug)]
#[derive(Debug, Clone)]
pub enum ASTNode {
    Assignment { var_name: String, value: Box<ASTNode> },
    Number(i64),
    BinaryOp { left: Box<ASTNode>, operator: Token, right: Box<ASTNode> },
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
    // Entferne und hole die Zahl auf der linken Seite
    if let Token::Number(left_val) = tokens.remove(0) {

        // Entferne und hole den Operator (+, -, *, /)
        if let operator = tokens.remove(0) {

            // Entferne und hole die Zahl auf der rechten Seite
            if let Token::Number(right_val) = tokens.remove(0) {

                return Some(ASTNode::BinaryOp {
                    left: Box::new(ASTNode::Number(left_val)),
                    operator,  // Nutze den Operator direkt
                    right: Box::new(ASTNode::Number(right_val)),
                });
            }
        }
    }
    None
}
