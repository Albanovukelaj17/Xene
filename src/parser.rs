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
    Switch {
        expression: Box<ASTNode>,
        cases: Vec<(ASTNode, ASTNode)>, // Each case has a value and a block
        default: Option<Box<ASTNode>>,  // Optional default block
    },
    List(Vec<ASTNode>),
    Print(Box<ASTNode>),

}

pub fn parse_assignment(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    println!("____Starting to parse assignment, current token: {:?},{:?}", tokens.get(0),tokens.get(1));

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


                    println!("______FInished Parsing");
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

                println!("___detected range: {}..{}", start, end);
                return Some(ASTNode::Range {
                    start: Box::new(ASTNode::Number(start)),
                    end: Box::new(ASTNode::Number(end)),
                });
            } else {
                println!("Error: Expected a number after `..` for the range end.");
                return None;
            }
        }

        // If no range, treat it as a primary number expression and continue parsing.
        return parse_binary_op_with_left(tokens, ASTNode::Number(start));
    }

    // Case 1: Parse an assignment expression (e.g., `x = x - 1`)
    if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
        tokens.remove(0); // Remove the variable name

        // Check if the next token is an equal sign (`=`)
        if let Some(Token::Equal) = tokens.get(0).cloned() {
            tokens.remove(0); // Remove the equal sign `=`

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

        // Parse binary expressions or return the identifier itself
        return parse_binary_expression_or_variable(tokens, var_name);
    }

    // Handle `print` statements
    if let Some(Token::Print) = tokens.get(0).cloned() {
        return parse_print(tokens);
    }

    // Parse primary expressions and potential binary operations
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


    println!("______starting prasing IF");
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

            println!("____ending parsing IF");
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
    println!("____starting parsing While");

    if let Some(Token::While) = tokens.get(0).cloned() {
        tokens.remove(0);  // Remove `while`

        println!("Parsing while condition, current token: {:?}", tokens.get(0));
        let condition = match parse_expression(tokens) {
            Some(cond) => cond,
            None => {
                println!("Error: Failed to parse while condition.");
                return None;
            }
        };

        println!("Parsed while condition: {:?}", condition);

        println!("Parsing while body, expecting `{{`.");
        let body = match parse_block(tokens) {
            Some(b) => b,
            None => {
                println!("Error: Failed to parse while body.");
                return None;
            }
        };

        println!("____ending parsing While");
        return Some(ASTNode::While {
            condition: Box::new(condition),
            body: Box::new(body),
        });
    }

    println!("Error: Not a 'while' statement.");
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

            // Try parsing a `while` statement first
            if let Some(statement) = parse_while(tokens) {
                statements.push(statement);
            }
            // Then try parsing an `if` statement
            else if let Some(statement) = parse_if(tokens) {
                statements.push(statement);
            }
            // Then try parsing an assignment or other expressions
            else if let Some(statement) = parse_assignment(tokens) {
                statements.push(statement);
            }
            // Optionally, parse other constructs like `for` or `switch` here
            else if let Some(statement) = parse_for(tokens) {
                statements.push(statement);
            } else if let Some(statement) = parse_switch(tokens) {
                statements.push(statement);
            }
            // Add other control structures here if needed.

            // Optionally, check for a semicolon after each statement
            if let Some(Token::Semicolon) = tokens.get(0).cloned() {
                tokens.remove(0); // Remove `;`
            } else {
                println!("Error: Expected a semicolon after the statement.");
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
pub fn parse_switch(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::Switch) = tokens.get(0).cloned() {
        tokens.remove(0); // Remove `switch`
        println!("_____Detected switch");

        // Parse the expression after `switch`
        let expression = parse_expression(tokens)?;
        println!("______Parsed switch expression");

        if let Some(Token::LeftBrace) = tokens.get(0).cloned() {
            tokens.remove(0); // Remove `{`
        } else {
            println!("Error: Expected `{{` after `switch` expression");
            return None;
        }

        let mut cases = Vec::new();
        let mut default_case = None;

        // Parse `case` statements and `default` block
        while let Some(token) = tokens.get(0).cloned() {
            match token {
                Token::Case => {
                    tokens.remove(0); // Remove `case`
                    let case_value = parse_expression(tokens)?;
                    println!("______Parsed case value: {:?}", case_value);

                    if let Some(Token::Colon) = tokens.get(0).cloned() {
                        tokens.remove(0); // Remove `:`
                    } else {
                        println!("Error: Expected `:` after `case` value");
                        return None;
                    }

                    // Try to parse a block or a single statement for the `case`
                    let case_block = if let Some(Token::LeftBrace) = tokens.get(0).cloned() {
                        parse_block(tokens)?
                    } else if let Some(statement) = parse_assignment(tokens) {
                        ASTNode::Block(vec![statement])
                    } else {
                        let statement = parse_expression(tokens)?;
                        ASTNode::Block(vec![statement])
                    };

                    cases.push((case_value, case_block));
                }
                Token::Default => {
                    tokens.remove(0); // Remove `default`

                    if let Some(Token::Colon) = tokens.get(0).cloned() {
                        tokens.remove(0); // Remove `:`
                    } else {
                        println!("Error: Expected `:` after `default`");
                        return None;
                    }

                    // Try to parse a block or a single statement for the `default`
                    default_case = Some(Box::new(if let Some(Token::LeftBrace) = tokens.get(0).cloned() {
                        parse_block(tokens)?
                    } else if let Some(statement) = parse_assignment(tokens) {
                        ASTNode::Block(vec![statement])
                    } else {
                        let statement = parse_expression(tokens)?;
                        ASTNode::Block(vec![statement])
                    }));
                }
                Token::RightBrace => {
                    tokens.remove(0); // Remove `}`
                    return Some(ASTNode::Switch {
                        expression: Box::new(expression),
                        cases,
                        default: default_case,
                    });
                }
                _ => {
                    println!("Error: Unexpected token in `switch` block");
                    return None;
                }
            }
        }

        println!("Error: Expected `}}` to close `switch` block");
        None
    } else {
        println!("Error: Not a `switch` statement.");
        None
    }
}
pub fn parse_list(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    if let Some(Token::LeftBracket) = tokens.get(0).cloned() {
        tokens.remove(0); // Entferne `[`, da wir jetzt eine Liste parsen

        let mut elements = Vec::new();

        while let Some(token) = tokens.get(0).cloned() {
            match token {
                Token::RightBracket => {
                    tokens.remove(0); // Entferne `]` und schließe die Liste ab
                    return Some(ASTNode::List(elements));
                }
                _ => {
                    // Versuche, jedes Element in der Liste zu parsen
                    if let Some(element) = parse_expression(tokens) {
                        elements.push(element);

                        // Überprüfe auf Komma zwischen Listenelementen
                        if let Some(Token::Comma) = tokens.get(0).cloned() {
                            tokens.remove(0); // Entferne `,` und gehe zum nächsten Element
                        } else if let Some(Token::RightBracket) = tokens.get(0).cloned() {
                            tokens.remove(0); // Schließe die Liste, wenn `]` kommt
                            return Some(ASTNode::List(elements));
                        } else {
                            println!("Error: Erwartetes `,` oder `]` nach Listenelement");
                            return None;
                        }
                    } else {
                        println!("Error: Konnte Listenelement nicht parsen");
                        return None;
                    }
                }
            }
        }
        println!("Error: Liste wurde nicht mit `]` geschlossen");
        None
    } else {
        println!("Error: Liste muss mit `[` beginnen");
        None
    }
}
