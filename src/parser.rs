use crate::lexer::Token;  // Import tokens from lexer

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Assignment { var_name: String, value: Box<ASTNode> },
    Number(i64),
    Identifier(String),
    BinaryOp { left: Box<ASTNode>, operator: Token, right: Box<ASTNode> },
    Block(Vec<ASTNode>),
    If {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    While {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    For {
        iterator: Box<ASTNode>,
        iterable: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    Range{
        start: Box<ASTNode>,
        end: Box<ASTNode>,
    },
    Print(Box<ASTNode>),

}

pub fn parse_assignment(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    println!("Starting to parse assignment, current token: {:?}", tokens.get(0));

    if let Some(Token::Var) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove `var`

        if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
            tokens.remove(0);  // Remove the variable name

            if let Some(Token::Equal) = tokens.get(0).cloned() {
                tokens.remove(0);  // Remove the equal sign `=`

                // Now parse an expression (e.g., `5 + 3`)
                if let Some(expression) = parse_expression(tokens) {
                    // Check for semicolon after the assignment
                    if let Some(Token::Semicolon) = tokens.get(0).cloned() {
                        tokens.remove(0);  // Remove the semicolon `;`
                        println!("Semicolon removed after assignment.");
                    } else {
                        println!("Optional: No semicolon found after assignment.");
                    }

                    // Return the assignment AST node
                    return Some(ASTNode::Assignment {
                        var_name: var_name.clone(),
                        value: Box::new(expression),
                    });
                } else {
                    println!("Error: Invalid expression in assignment.");
                    return None;
                }
            }
        }
    }

    println!("Failed to parse assignment.");
    None
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    println!("Starting parse_expression, current token: {:?}", tokens.get(0));

    // Case 0: Parse a range expression (e.g., `1..10`)
    if let Some(Token::Number(start)) = tokens.get(0).cloned() {
        tokens.remove(0); // Remove the start number

        // Check if the next token is `..`
        if let Some(Token::Range) = tokens.get(0).cloned() {
            tokens.remove(0); // Remove the range operator `..`

            // Check if there's a number for the end value of the range
            if let Some(Token::Number(end)) = tokens.get(0).cloned() {
                tokens.remove(0); // Remove the end number

                // Create the Range node
                println!("___detected range: {}..{}", start, end);
                return Some(ASTNode::Range {
                    start: Box::new(ASTNode::Number(start)),
                    end: Box::new(ASTNode::Number(end)),
                });
            } else {
                println!("Error: Expected a number after `..` for the range end.");
                return None;
            }
        } else {
            // If there's no `..`, treat it as a number expression
            return Some(ASTNode::Number(start));
        }
    }

    // Case 1: Parse an assignment expression (e.g., `x = x - 1`)
    if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
        tokens.remove(0); // Remove the variable name (e.g., `x`)

        // Check if the next token is an equal sign (`=`)
        if let Some(Token::Equal) = tokens.get(0).cloned() {
            tokens.remove(0); // Remove the equal sign `=`

            // Parse the right-hand side of the assignment (e.g., `x - 1`)
            if let Some(right_expr) = parse_expression(tokens) {
                println!("Parsed assignment: {} = {:?}", var_name, right_expr);
                return Some(ASTNode::Assignment {
                    var_name,
                    value: Box::new(right_expr),
                });
            } else {
                println!("Error: Expected an expression after `=`");
                return None;
            }
        }

        // Case 2: Parse binary expressions or return the identifier itself
        return parse_binary_expression_or_variable(tokens, var_name);
    }

    // Case 3: Handle `print` statements (e.g., `print(x)`)
    if let Some(Token::Print) = tokens.get(0).cloned() {
        return parse_print(tokens);
    }

    // Case 4: Parse primary expressions and potential binary operations (e.g., `5 + 3`)
    if let Some(left) = parse_primary_expression(tokens) {
        return parse_binary_op_with_left(tokens, left);
    }

    println!("No valid expression found, returning `None`");
    None
}


pub fn parse_print(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    tokens.remove(0);  // Remove `print`

    // Expect an opening parenthesis `(` after `print`
    if let Some(Token::LeftParen) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove `(`

        // Expect the expression inside the parentheses (e.g., the variable to print)
        if let Some(expression) = parse_primary_expression(tokens) {
            // Expect the closing parenthesis `)`
            if let Some(Token::RightParen) = tokens.get(0).cloned() {
                tokens.remove(0);  // Remove `)`
                // Now expect a semicolon `;` to end the statement
                if let Some(Token::Semicolon) = tokens.get(0).cloned() {
                    tokens.remove(0);  // Remove `;`
                    return Some(ASTNode::Print(Box::new(expression)));
                } else {
                    println!("Error: Missing semicolon after `print` statement");
                    return None;
                }
            } else {
                println!("Error: Missing closing parenthesis `)` after expression");
                return None;
            }
        } else {
            println!("Error: Invalid expression inside `print`");
            return None;
        }
    } else {
        println!("Error: Missing opening parenthesis `(` after `print`");
        return None;
    }
}

pub fn parse_if(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::If) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove `if`
        let condition = parse_expression(tokens)?;
        if let Some(Token::LeftBrace) = tokens.get(0) {
            let then_branch = parse_block(tokens)?;
            let else_branch = if let Some(Token::Else) = tokens.get(0) {
                tokens.remove(0);  // Remove `else`
                if let Some(Token::LeftBrace) = tokens.get(0) {
                    Some(Box::new(parse_block(tokens)?))
                } else {
                    println!("Error: `else` block must start with `{{`");
                    return None;
                }
            } else {
                None
            };
            return Some(ASTNode::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch,
            });
        } else {
            println!("Error: `then` block must start with `{{`.");
            return None;
        }
    }
    None
}

pub fn parse_while(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::While) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove `while`
        let condition = parse_expression(tokens)?;
        let body = parse_block(tokens)?;
        return Some(ASTNode::While {
            condition: Box::new(condition),
            body: Box::new(body),
        });
    }
    None
}

pub fn parse_block(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::LeftBrace) = tokens.get(0).cloned() {
        tokens.remove(0); // Remove `{`
        let mut statements = Vec::new();

        // Parse each statement until we encounter a `}`
        while let Some(token) = tokens.get(0) {
            if let Token::RightBrace = token {
                tokens.remove(0); // Remove `}`
                return Some(ASTNode::Block(statements));
            }

            // Try parsing a statement inside the block
            if let Some(statement) = parse_expression(tokens) {
                statements.push(statement);

                // Optionally, check for a semicolon after each statement
                if let Some(Token::Semicolon) = tokens.get(0).cloned() {
                    tokens.remove(0); // Remove `;`
                }
            } else {
                println!("Error: Failed to parse a statement inside the block.");
                return None;
            }
        }

        println!("Error: Block was not properly closed with `}}`.");
        return None;
    }

    println!("Error: Block must start with `{{`.");
    None
}

pub fn parse_binary_op(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    let left = parse_primary_expression(tokens)?;
    if let Some(operator) = tokens.get(0).cloned() {
        match operator {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::GreaterThan | Token::LessThan => {
                tokens.remove(0);  // Remove the operator
                let right = parse_primary_expression(tokens)?;
                return Some(ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                });
            }
            _ => return Some(left),
        }
    }
    None
}

pub fn parse_primary_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::LeftParen) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove `(`
        let expression = parse_expression(tokens);
        if let Some(Token::RightParen) = tokens.get(0).cloned() {
            tokens.remove(0);  // Remove `)`
            return expression;
        } else {
            println!("Error: Expected closing paren `)`");
            return None;
        }
    }

    if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove identifier
        return Some(ASTNode::Identifier(var_name));
    }

    if let Some(Token::Number(value)) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove number
        return Some(ASTNode::Number(value));
    }

    println!("Error: No valid primary expression found");
    None
}

pub fn parse_binary_op_with_left(tokens: &mut Vec<Token>, left: ASTNode) -> Option<ASTNode> {
    if let Some(operator) = tokens.get(0).cloned() {
        match operator {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::GreaterThan | Token::LessThan => {
                tokens.remove(0);  // Remove the operator
                if let Some(right) = parse_primary_expression(tokens) {
                    return Some(ASTNode::BinaryOp {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    });
                }
            }
            _ => {}
        }
    }
    Some(left)
}

fn parse_binary_expression_or_variable(tokens: &mut Vec<Token>, var_name: String) -> Option<ASTNode> {
    println!("Detected variable or potential binary expression: {}", var_name);

    // If the next token is a binary operator, treat it as a binary expression
    if let Some(operator) = tokens.get(0).cloned() {
        match operator {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide |
            Token::GreaterThan | Token::LessThan | Token::GreaterEqual | Token::LessEqual => {
                tokens.remove(0);  // Remove the operator

                // Parse the right-hand side of the binary operation
                if let Some(right_expr) = parse_primary_expression(tokens) {
                    println!("Parsed right-hand side of binary operation: {:?}", right_expr);
                    return Some(ASTNode::BinaryOp {
                        left: Box::new(ASTNode::Identifier(var_name)),
                        operator,
                        right: Box::new(right_expr),
                    });
                } else {
                    println!("Error: Expected right-hand side expression after operator");
                    return None;
                }
            }
            _ => {
                println!("No valid operator found, treating as a simple variable.");
                return Some(ASTNode::Identifier(var_name));
            }
        }
    }

    println!("No operator found, returning simple variable: {}", var_name);
    Some(ASTNode::Identifier(var_name))
}

pub fn parse_for(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::For) = tokens.get(0).cloned() {
        tokens.remove(0); // Remove `for`
        println!("_____Detected for");
        // Parse the loop variable (iterator)
        let iterator = match tokens.get(0).cloned() {
            Some(Token::Identifier(var_name)) => {
                tokens.remove(0); // Remove the identifier
                ASTNode::Identifier(var_name) }
            _ => {
                println!("Error: Expected identifier in for loop.");
                return None;
            }
        };
        println!("______Detected Identifier i for example ");

        // Expect the `in` keyword
        if let Some(Token::In) = tokens.get(0).cloned() {
            tokens.remove(0); // Remove `in`
            println!("______Detected  in  ");
        } else {
            println!("Error: Expected 'in' in for loop.");
            return None;
        }

        // Parse the iterable (e.g., a range)
        let iterable = parse_expression(tokens)?; // This should handle range expressions like `1..10`
        println!("___detected 1..10,{:?}", iterable);
        // Parse the loop body
        let body = parse_block(tokens)?;
        println!("____detected block,{:?}",body);
        // Return the ASTNode for the for loop
        return Some(ASTNode::For {
            iterator: Box::new(iterator),
            iterable: Box::new(iterable),
            body: Box::new(body),
        });
    }

    println!("Error: Not a 'for' loop.");
    None
}
