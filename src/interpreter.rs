use std::collections::HashMap;
use crate::lexer::Token;
use crate::parser::ASTNode;
use crate::parser::{parse_if, parse_while};

pub fn interpret(ast: ASTNode, env: &mut HashMap<String, i64>) {
    match ast {
        // Zuweisung von Variablen (z.B. `x = x - 1`)
        ASTNode::Assignment { var_name, value } => {
            let new_val = evaluate_expression(*value, env);
            env.insert(var_name.clone(), new_val);
        }

        // Print-Anweisung
        ASTNode::Print(expr) => {
            let val = evaluate_expression(*expr, env);
            println!("print: {}", val);  // Gib den Wert aus
        }

        // If-Anweisung
        ASTNode::If { condition, then_branch, else_branch } => {
            if evaluate_condition(*condition, env) {
                interpret(*then_branch, env);
            } else if let Some(else_branch) = else_branch {
                interpret(*else_branch, env);
            }
        }

        // While-Anweisung
        ASTNode::While { condition, body } => {
            while evaluate_condition(*condition.clone(), env) {
                interpret(*body.clone(), env); // Führe den Schleifenbody aus
            }
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


mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::{parse_assignment, parse_expression};

    #[test]
    fn test_interpreter_with_assignment() {
        let input = "var x = 10;";
        let mut tokens = tokenize(input);
        let mut env = HashMap::new();
        if let Some(ast) = parse_assignment(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 10);
    }

    #[test]
    fn test_interpreter_with_expression() {
        let input = "x = x - 1;";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10);
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 9);
    }

    #[test]
    fn test_interpreter_if_else() {
        let input = "var x = 4; if x > 5 { print(1); } else { print(0); }";
        let mut tokens = tokenize(input);
        let mut env = HashMap::new();

        // Parse and execute the assignment
        if let Some(ast) = parse_assignment(&mut tokens) {
            interpret(ast, &mut env);
        }

        // Parse and execute the if-else statement
        if let Some(ast) = parse_if(&mut tokens) {
            interpret(ast, &mut env);
        }

        // In this case, since x = 4, the else branch should be taken, printing 0.
        assert_eq!(env.get("x"), Some(&4));
    }

    #[test]
    fn test_interpreter_while_loop() {
        let input = "var x = 10; while x > 5 { x = x - 1; }";
        let mut tokens = tokenize(input);
        let mut env = HashMap::new();

        // Parse and execute the assignment
        if let Some(ast) = parse_assignment(&mut tokens) {
            interpret(ast, &mut env);
        }

        // Parse and execute the while loop
        if let Some(ast) = parse_while(&mut tokens) {
            interpret(ast, &mut env);
        }

        // Nach der Schleife sollte x gleich 5 sein
        assert_eq!(env.get("x"), Some(&5));
    }



}