use crate::lexer::Token;  // Importiere die Tokens aus dem Lexer

#[derive(Debug)]
pub enum ASTNode {
    Assignment { var_name: String, value: Box<ASTNode> },
    Number(i64),
    BinaryOp { left: Box<ASTNode>, operator: Token, right: Box<ASTNode> },
}

pub fn parse_assignment(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::Var) = tokens.get(0) {
        tokens.remove(0);  // Entferne `var`
        if let Some(Token::Identifier(var_name)) = tokens.get(0) {
            tokens.remove(0);  // Entferne den Variablennamen
            if let Some(Token::Equal) = tokens.get(0) {
                tokens.remove(0);  // Entferne das Gleichheitszeichen `=`
                if let Some(Token::Number(value)) = tokens.get(0) {
                    tokens.remove(0);  // Entferne die Zahl
                    return Some(ASTNode::Assignment {
                        var_name: var_name.clone(),
                        value: Box::new(ASTNode::Number(*value)),
                    });
                }
            }
        }
    }
    None
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    // Erwarte eine Zahl auf der linken Seite
    if let Some(Token::Number(left_val)) = tokens.get(0) {
        tokens.remove(0); // Entferne die Zahl

        // Erwarte einen Operator (+, -, *, /)
        if let Some(operator) = tokens.get(0).cloned() {

            tokens.remove(0); // Entferne den Operator

            // Erwarte eine Zahl auf der rechten Seite
            if let Some(Token::Number(right_val)) = tokens.get(0) {
                tokens.remove(0); // Entferne die Zahl

                return Some(ASTNode::BinaryOp {
                    left: Box::new(ASTNode::Number(*left_val)),
                    operator: operator.cloned(),
                    right: Box::new(ASTNode::Number(*right_val)),
                });
            }
        }
    }
    None
}

