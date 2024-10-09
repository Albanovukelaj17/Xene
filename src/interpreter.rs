use std::collections::HashMap;
use crate::lexer::Token;
use crate::parser::ASTNode;

pub fn interpret(ast: ASTNode, env: &mut HashMap<String, i64>) {
    match ast {
        // Zuweisung von Variablen (z.B. `x = x - 1`)
        ASTNode::Assignment { var_name, value } => {
            // Überprüfen, ob die Variable bereits existiert
            if let Some(&old_val) = env.get(&var_name) {
                println!("Alter Wert von {}: {}", var_name, old_val);

                // Wert der rechten Seite auswerten (z.B. `x - 1`)
                let new_val = evaluate_expression(*value, env);
                println!("Berechne neuen Wert von {}: {}", var_name, new_val);

                // Aktualisiere den Wert in der Umgebung
                env.insert(var_name.clone(), new_val);
                println!("Wert von {} nach Zuweisung: {}", var_name, new_val);
            } else {
                // Falls die Variable neu ist, weise den Wert zu
                let val = evaluate_expression(*value, env);
                println!("Zuweisung (neue Variable): {} = {}", var_name, val);
                env.insert(var_name.clone(), val);
            }
        }

        // Print-Anweisung
        ASTNode::Print(expr) => {
            let val = evaluate_expression(*expr, env);
            println!("print: {}", val);  // Gib den Wert aus
        }

        // Weitere Anweisungen wie `if`, `while`, etc.
        _ => {}
    }
}

// Hilfsfunktion zur Auswertung von Ausdrücken
pub fn evaluate_expression(expr: ASTNode, env: &mut HashMap<String, i64>) -> i64 {
    match expr {
        // Wenn der Ausdruck eine Zahl ist, gib diese zurück
        ASTNode::Number(val) => val,

        // Wenn der Ausdruck eine Variable ist, hole ihren Wert aus der Umgebung
        ASTNode::Identifier(var_name) => {
            if let Some(&val) = env.get(&var_name) {
                println!("Wert von {}: {}", var_name, val);  // Debugging-Ausgabe
                val
            } else {
                panic!("Unbekannte Variable: {}", var_name);
            }
        }

        // Wenn der Ausdruck eine binäre Operation ist, führe die Operation aus
        ASTNode::BinaryOp { left, operator, right } => {
            let left_val = evaluate_expression(*left, env);  // Linken Ausdruck auswerten
            let right_val = evaluate_expression(*right, env);  // Rechten Ausdruck auswerten

            println!("Berechne: {} {:?} {}", left_val, operator, right_val);  // Debugging-Ausgabe

            match operator {
                Token::Plus => left_val + right_val,
                Token::Minus => left_val - right_val,
                Token::Multiply => left_val * right_val,
                Token::Divide => left_val / right_val,
                _ => panic!("Unbekannter Operator"),
            }
        }

        _ => panic!("Ungültiger Ausdruck"),
    }
}

// Funktion zur Auswertung von Bedingungen
pub fn evaluate_condition(condition: ASTNode, env: &mut HashMap<String, i64>) -> bool {
    match condition {
        ASTNode::Number(val) => val != 0,
        ASTNode::Identifier(var_name) => {
            if let Some(&val) = env.get(&var_name) {
                println!("Bedingung für {}: {}", var_name, val != 0);  // Debugging
                val != 0
            } else {
                false
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
        _ => false,
    }
}


//      var x = 10;
//      while x > 5 { print(x); x = x - 1; }
