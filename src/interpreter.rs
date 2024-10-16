use std::collections::HashMap;
use crate::lexer::Token;
use crate::parser::ASTNode;
use crate::parser::{parse_if, parse_while};

pub fn interpret(ast: ASTNode, env: &mut HashMap<String, i64>) {
    match ast {
        // Variable assignment (e.g., `x = x - 1`)
        ASTNode::Assignment { var_name, value } => {
            let new_val = evaluate_expression(*value, env);
            println!("Assigning {} to {}", new_val, var_name);
            env.insert(var_name.clone(), new_val);  // Update the variable in the environment
        }

        // Print statement
        ASTNode::Print(expr) => {
            let val = evaluate_expression(*expr, env);
            println!("print: {}", val);  // Print the evaluated value
        }

        // If statement
        ASTNode::If { condition, then_branch, else_branch } => {
            if evaluate_condition(*condition, env) {
                interpret(*then_branch, env);
            } else if let Some(else_branch) = else_branch {
                interpret(*else_branch, env);
            }
        }

        // While statement
        ASTNode::While { condition, body } => {
            // Loop until the condition becomes false
            while evaluate_condition(*condition.clone(), env) {
                println!("While loop condition is true. Current env: {:?}", env);

                // Make sure we process each statement in the body separately
                if let ASTNode::Block(ref mut statements) = *body.clone() {
                    for statement in statements.iter() {
                        interpret(statement.clone(), env);  // Execute each statement in the block
                    }
                } else {
                    interpret(*body.clone(), env);
                }

                // Re-evaluate the condition with the updated environment
                if !evaluate_condition(*condition.clone(), env) {
                    println!("Exiting loop, condition is now false.");
                    break;
                }
            }
        }

        // Block of multiple statements
        ASTNode::Block(statements) => {
            for statement in statements {
                interpret(statement, env);  // Interpret each statement in the block
            }
        }

        _ => {}
    }
}

// Hilfsfunktion zur Auswertung von Ausdrücken
pub fn evaluate_expression(expr: ASTNode, env: &mut HashMap<String, i64>) -> i64 {
    match expr {
        // Number
        ASTNode::Number(val) => val,

        // Identifier (variable)
        ASTNode::Identifier(var_name) => {
            if let Some(&val) = env.get(&var_name) {
                println!("Wert von {}: {}", var_name, val);  // Debugging output for variable
                val
            } else {
                panic!("Unbekannte Variable: {}", var_name);
            }
        }

        // Binary operation (e.g., `x - 1`)
        ASTNode::BinaryOp { left, operator, right } => {
            let left_val = evaluate_expression(*left, env);  // Evaluate left operand
            let right_val = evaluate_expression(*right, env);  // Evaluate right operand

            println!("Berechne: {} {:?} {}", left_val, operator, right_val);  // Debugging

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
                println!("Condition for {}: {}", var_name, val > 0);
                val != 0
            } else {
                false
            }
        }
        ASTNode::BinaryOp { left, operator, right } => {
            let left_val = evaluate_expression(*left, env);
            let right_val = evaluate_expression(*right, env);
            println!("Evaluating condition: {} {:?} {}", left_val, operator, right_val);  // Debugging


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

        println!("Tokens before assignment parsing: {:?}", tokens);  // Check the token stream before parsing

        let mut env = HashMap::new();

        // Parse and execute the assignment
        if let Some(ast) = parse_assignment(&mut tokens) {
            println!("Interpreting assignment: {:?}", ast);
            interpret(ast, &mut env);
        } else {
            println!("Failed to parse assignment.");  // Debug if assignment parsing fails
        }

        println!("Tokens after assignment interpretation: {:?}", tokens);  // Check remaining tokens after assignment

        // Parse and execute the while loop
        if let Some(ast) = parse_while(&mut tokens) {
            println!("Interpreting while loop: {:?}", ast);
            interpret(ast, &mut env);
        } else {
            println!("Failed to parse the while loop.");  // Debug if while loop parsing fails
        }

        // Print the final value of x in the environment for debugging
        println!("Final value of x in env: {:?}", env.get("x"));

        // After the loop, `x` should be 5
        assert_eq!(env.get("x"), Some(&5));
    }

}