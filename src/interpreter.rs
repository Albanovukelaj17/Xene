use std::collections::HashMap;
use crate::lexer::Token;  // Damit der Token-Typ bekannt ist
use crate::parser::ASTNode;  // Damit die AST-Struktur bekannt ist

pub fn interpret(ast: ASTNode, env: &mut HashMap<String, i64>) {
    match ast {
        ASTNode::Assignment { var_name, value } => {
            if let ASTNode::Number(val) = *value {
                env.insert(var_name, val);
            }
        }
        ASTNode::BinaryOp { left, operator, right } => {
            if let ASTNode::Number(left_val) = *left {
                if let ASTNode::Number(right_val) = *right {
                    let result = match operator {
                        Token::Plus => left_val + right_val,
                        Token::Minus => left_val - right_val,
                        Token::Multiply => left_val * right_val,
                        Token::Divide => left_val / right_val,
                        _ => 0, // Andere Operatoren ignorieren
                    };
                    println!("Ergebnis: {}", result);
                }
            }
        }
        ASTNode::If { condition, then_branch, else_branch } => {
            if evaluate_condition(*condition, env) {  // Wenn die Bedingung wahr ist
                interpret(*then_branch, env);  // Führe den `then`-Zweig aus
            } else if let Some(else_branch) = else_branch {  // Sonst, falls ein `else`-Zweig existiert
                interpret(*else_branch, env);
            }
        }
        _ => {}  // Weitere Knoten behandeln (wie Schleifen)
    }
}

