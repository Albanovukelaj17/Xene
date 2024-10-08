use std::collections::HashMap;
use crate::lexer::Token;  // Damit der Token-Typ bekannt ist
use crate::parser::ASTNode;  // Damit die AST-Struktur bekannt ist

pub fn interpret(ast: ASTNode, env: &mut HashMap<String, i64>) {
    match ast {
        // Zuweisung von Variablen
        ASTNode::Assignment { var_name, value } => {
            let val = evaluate_expression(*value, env);  // Berechne den Wert des Ausdrucks
            env.insert(var_name, val);  // Setze den neuen Wert der Variablen in der Umgebung
        }

        // Verarbeitung des `print`-Statements
        ASTNode::Print(expr) => {
            let val = evaluate_expression(*expr, env);  // Berechne den Ausdruck
            println!("{}", val);  // Gib den Wert der Variablen aus
        }

        // Verarbeitung der `while`-Schleife
        ASTNode::While { condition, body } => {
            while evaluate_condition(*condition.clone(), env) {  // Auswertung der Bedingung
                interpret(*body.clone(), env);  // Führe den Body der Schleife aus
            }
        }

        // Verarbeitung von Blöcken
        ASTNode::Block(statements) => {
            for statement in statements {
                interpret(statement, env);  // Führe jede Anweisung im Block aus
            }
        }

        // Verarbeitung von `if`-Statements
        ASTNode::If { condition, then_branch, else_branch } => {
            if evaluate_condition(*condition, env) {  // Wenn die Bedingung wahr ist
                interpret(*then_branch, env);  // Führe den `then`-Zweig aus
            } else if let Some(else_branch) = else_branch {  // Sonst, falls ein `else`-Zweig existiert
                interpret(*else_branch, env);
            }
        }

        _ => {}
    }
}

// Hilfsfunktion zur Auswertung von Ausdrücken (Zahlen und Variablen)
pub fn evaluate_expression(expr: ASTNode, env: &mut HashMap<String, i64>) -> i64 {
    match expr {
        ASTNode::Number(val) => val,  // Gib die Zahl direkt zurück
        ASTNode::Identifier(var_name) => {
            if let Some(&val) = env.get(&var_name) {
                val  // Gib den Wert der Variablen zurück
            } else {
                panic!("Fehler: Unbekannte Variable `{}`", var_name);
            }
        }
        ASTNode::BinaryOp { left, operator, right } => {
            let left_val = evaluate_expression(*left, env);
            let right_val = evaluate_expression(*right, env);

            match operator {
                Token::Plus => left_val + right_val,
                Token::Minus => left_val - right_val,
                Token::Multiply => left_val * right_val,
                Token::Divide => left_val / right_val,
                _ => panic!("Fehler: Unbekannter Operator"),
            }
        }
        _ => panic!("Fehler: Ungültiger Ausdruck"),
    }
}

// Funktion zur Auswertung von Bedingungen
pub fn evaluate_condition(condition: ASTNode, env: &mut HashMap<String, i64>) -> bool {
    match condition {
        ASTNode::Number(val) => val != 0,  // Wenn die Zahl `0` ist, ist die Bedingung falsch
        ASTNode::Identifier(var_name) => {
            if let Some(&val) = env.get(&var_name) {
                val != 0  // Wenn der Wert nicht `0` ist, ist die Bedingung `true`
            } else {
                false  // Wenn die Variable nicht existiert, ist die Bedingung `false`
            }
        }
        ASTNode::BinaryOp { left, operator, right } => {
            let left_val = evaluate_expression(*left, env);
            let right_val = evaluate_expression(*right, env);

            match operator {
                Token::GreaterThan => left_val > right_val,
                Token::LessThan => left_val < right_val,
                Token::GreaterEqual => left_val >= right_val,
                Token::LessEqual => left_val <= right_val,
                _ => false,
            }
        }
        _ => false,  // Alle anderen Bedingungen sind ungültig
    }
}
