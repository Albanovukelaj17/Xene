use std::collections::HashMap;
use crate::lexer::Token;
use crate::parser::ASTNode;
use crate::parser::{parse_if, parse_while};

// Function to interpret the given AST node.
pub fn interpret(ast: ASTNode, env: &mut HashMap<String, i64>) {
    match ast {
        // Variable assignment (e.g., `x = x - 1`)
        ASTNode::Assignment { var_name, value } => {
            let new_val = evaluate_expression(*value, env);
            println!("Assigning value {} to variable {}", new_val, var_name);
            env.insert(var_name.clone(), new_val);  // Update the variable in the environment
        }

        // Print statement
        ASTNode::Print(expr) => {
            let val = evaluate_expression(*expr, env);
            println!("Print statement output: {}", val);  // Print the evaluated value
        }

        // If statement
        ASTNode::If { condition, then_branch, else_branch } => {
            let condition_result = evaluate_condition(*condition.clone(), env);
            println!("Evaluating IF statement, condition: {:?}, result: {}", condition, condition_result);

            if condition_result {
                println!("Executing THEN branch of IF statement.");
                interpret(*then_branch, env);
            } else if let Some(else_branch) = else_branch {
                println!("Executing ELSE branch of IF statement.");
                interpret(*else_branch, env);
            } else {
                println!("Condition was false and no ELSE branch.");
            }
        }

        // While statement
        ASTNode::While { condition, body } => {
            println!("Starting WHILE loop with condition: {:?}", condition);
            while evaluate_condition(*condition.clone(), env) {
                println!("WHILE loop condition is true. Current environment: {:?}", env);

                // Interpret each statement inside the loop's body.
                if let ASTNode::Block(ref statements) = *body.clone() {
                    for statement in statements {
                        println!("Executing statement in WHILE loop body: {:?}", statement);
                        interpret(statement.clone(), env);
                    }
                } else {
                    interpret(*body.clone(), env);
                }

                // Re-evaluate the condition after each iteration.
                if !evaluate_condition(*condition.clone(), env) {
                    println!("Exiting WHILE loop, condition is now false.");
                    break;
                }
            }
            println!("Exited WHILE loop.");
        }

        // Block of multiple statements
        ASTNode::Block(statements) => {
            println!("Executing block of statements.");
            for statement in statements {
                interpret(statement, env);
            }
            println!("Finished executing block.");
        }

        // For loop
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

                    // Loop over the range and update the iterator variable in the environment.
                    for i in start_val..end_val {
                        println!("For loop iteration: {} = {}", var_name, i);
                        env.insert(var_name.clone(), i);

                        // Interpret the body of the `for` loop for each iteration.
                        interpret(*body.clone(), env);
                    }

                    // Remove the iterator from the environment after the loop finishes.
                    env.remove(&var_name);
                } else {
                    println!("Error: Expected a range as the iterable in the `for` loop.");
                }
            } else {
                println!("Error: Expected an identifier as the iterator in the `for` loop.");
            }
        }

        // Switch statement
        ASTNode::Switch { expression, cases, default } => {
            let expr_val = evaluate_expression(*expression, env);
            println!("Evaluating SWITCH statement with expression value: {}", expr_val);
            let mut matched = false;

            for (case_value, case_block) in cases {
                let case_val = evaluate_expression(case_value, env);
                println!("Comparing case value: {} with expression value: {}", case_val, expr_val);

                if case_val == expr_val {
                    matched = true;
                    println!("Matched case value. Executing case block.");
                    interpret(case_block, env);
                    break;
                }
            }

            if !matched {
                if let Some(default_block) = default {
                    println!("No case matched. Executing default block.");
                    interpret(*default_block, env);
                } else {
                    println!("No case matched and no default block.");
                }
            }
        }
        ASTNode::List(list) => {
            let evaluated_list: Vec<i64> = list.into_iter()
                .map(|element| evaluate_expression(element, env))
                .collect();
            println!("Evaluated list: {:?}", evaluated_list);
        }

        _ => {
            println!("Unrecognized AST node: {:?}", ast);
        }
    }
}

// Function to evaluate expressions.
pub fn evaluate_expression(expr: ASTNode, env: &mut HashMap<String, i64>) -> i64 {
    match expr {
        ASTNode::Number(val) => val,

        ASTNode::Identifier(var_name) => {
            if let Some(&val) = env.get(&var_name) {
                println!("Retrieved value of variable {}: {}", var_name, val);
                val
            } else {
                println!("Error: Undefined variable {}", var_name);
                0 // Default to zero or handle this error differently.
            }
        }

        ASTNode::BinaryOp { left, operator, right } => {
            let left_val = evaluate_expression(*left, env);
            let right_val = evaluate_expression(*right, env);

            println!("Evaluating binary operation: {} {:?} {}", left_val, operator, right_val);

            match operator {
                Token::Plus => left_val + right_val,
                Token::Minus => left_val - right_val,
                Token::Multiply => left_val * right_val,
                Token::Divide => left_val / right_val,
                _ => {
                    println!("Error: Unknown operator {:?}", operator);
                    0
                }
            }
        }

        _ => {
            println!("Error: Unsupported expression type {:?}", expr);
            0
        }
    }
}

// Function to evaluate conditions (returns a boolean).
pub fn evaluate_condition(condition: ASTNode, env: &mut HashMap<String, i64>) -> bool {
    match condition {
        ASTNode::Number(val) => val != 0,

        ASTNode::Identifier(var_name) => {
            if let Some(&val) = env.get(&var_name) {
                println!("Condition for {}: {}", var_name, val != 0);
                val != 0
            } else {
                println!("Condition: Undefined variable {}", var_name);
                false
            }
        }

        ASTNode::BinaryOp { left, operator, right } => {
            let left_val = evaluate_expression(*left, env);
            let right_val = evaluate_expression(*right, env);
            println!("Evaluating condition: {} {:?} {}", left_val, operator, right_val);

            match operator {
                Token::GreaterThan => left_val > right_val,
                Token::LessThan => left_val < right_val,
                Token::GreaterEqual => left_val >= right_val,
                Token::LessEqual => left_val <= right_val,
                _ => {
                    println!("Error: Unsupported comparison operator {:?}", operator);
                    false
                }
            }
        }

        _ => {
            println!("Error: Unsupported condition type {:?}", condition);
            false
        }
    }
}


