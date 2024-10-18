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
        ASTNode::For {
            iterator,
            iterable,
            body,
        } => {
            // Assume `iterator` is an `ASTNode::Identifier`
            if let ASTNode::Identifier(var_name) = *iterator {
                // Assume `iterable` is an `ASTNode::Range` with `start` and `end`
                if let ASTNode::Range { start, end } = *iterable {
                    let start_val = evaluate_expression(*start, env);
                    let end_val = evaluate_expression(*end, env);

                    println!("For loop: iterating from {} to {}", start_val, end_val);

                    // Loop over the range and update the iterator variable in the environment
                    for i in start_val..end_val {
                        println!("For loop iteration: {} = {}", var_name, i);
                        env.insert(var_name.clone(), i);

                        // Interpret the body of the `for` loop for each iteration
                        interpret(*body.clone(), env);
                    }

                    // Remove the iterator from the environment after the loop finishes
                    env.remove(&var_name);
                } else {
                    println!("Error: Expected a range as the iterable in the `for` loop.");
                }
            } else {
                println!("Error: Expected an identifier as the iterator in the `for` loop.");
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
                println!("Error: Unbekannte Variable: {}", var_name);  // Handle undefined variable
                0 // Return a default value (optional) or handle the error in another way
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



