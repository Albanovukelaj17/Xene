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

    if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove the variable

        if let Some(Token::Equal) = tokens.get(0).cloned() {
            tokens.remove(0);  // Remove the equal sign `=`

            // Parse the right-hand side expression (e.g., `x - 1`)
            if let Some(right_expr) = parse_binary_op(tokens) {
                return Some(ASTNode::Assignment {
                    var_name,
                    value: Box::new(right_expr),
                });
            } else {
                println!("Error: Expected an expression after `=`");
                return None;
            }
        } else {
            // Handle a binary operation or return a simple variable identifier
            return parse_binary_expression_or_variable(tokens, var_name);
        }
    }

    // Handle `print` statements
    if let Some(Token::Print) = tokens.get(0).cloned() {
        return parse_print(tokens);
    }

    // Handle binary operations or primary expressions
    if let Some(left) = parse_primary_expression(tokens) {
        return parse_binary_op_with_left(tokens, left);
    }

    println!("No valid expression found, returning `None`");
    None
}

pub fn parse_print(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    tokens.remove(0);  // Remove `print`
    if let Some(Token::LeftParen) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove `(`
        if let Some(expression) = parse_primary_expression(tokens) {
            if let Some(Token::RightParen) = tokens.get(0).cloned() {
                tokens.remove(0);  // Remove `)`
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
    if let Some(Token::LeftBrace) = tokens.get(0) {
        tokens.remove(0);  // Remove `{`
        let mut statements = Vec::new();

        while let Some(token) = tokens.get(0) {
            if let Token::RightBrace = token {
                tokens.remove(0);  // Remove `}`
                return Some(ASTNode::Block(statements));
            }

            if let Some(statement) = parse_expression(tokens) {
                statements.push(statement);
                if let Some(Token::Semicolon) = tokens.get(0) {
                    tokens.remove(0);  // Remove `;`
                }
            } else {
                println!("Error parsing statement in block.");
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

pub fn parse_binary_expression_or_variable(tokens: &mut Vec<Token>, var_name: String) -> Option<ASTNode> {
    if let Some(operator) = tokens.get(0).cloned() {
        match operator {
            Token::GreaterThan | Token::LessThan | Token::GreaterEqual | Token::LessEqual | Token::Plus | Token::Minus => {
                tokens.remove(0);  // Remove the operator
                if let Some(right_expr) = parse_primary_expression(tokens) {
                    return Some(ASTNode::BinaryOp {
                        left: Box::new(ASTNode::Identifier(var_name)),
                        operator,
                        right: Box::new(right_expr),
                    });
                }
            }
            _ => return Some(ASTNode::Identifier(var_name)),
        }
    }
    Some(ASTNode::Identifier(var_name))
}

pub fn parse_for(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::For) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove `for`

        // Expect an identifier (loop variable)
        let iterator = match tokens.get(0).cloned() {
            Some(Token::Identifier(var_name)) => {
                tokens.remove(0);  // Remove the identifier
                ASTNode::Identifier(var_name)
            }
            _ => {
                println!("Error: Expected identifier in for loop.");
                return None;
            }
        };

        // Expect the `in` keyword
        if let Some(Token::In) = tokens.get(0).cloned() {
            tokens.remove(0);  // Remove `in`
        } else {
            println!("Error: Expected 'in' in for loop.");
            return None;
        }

        // Parse the iterable expression (e.g., range or collection)
        let iterable = parse_expression(tokens)?;

        // Expect a block `{}` for the loop body
        let body = parse_block(tokens)?;

        return Some(ASTNode::For {
            iterator: Box::new(iterator),
            iterable: Box::new(iterable),
            body: Box::new(body),
        });
    }
    None
}
