use crate::lexer::Token;  // Importiere die Tokens aus dem Lexer

        #[derive(Debug, Clone, PartialEq)]

        pub enum ASTNode {
            Assignment { var_name: String, value: Box<ASTNode> },
            Number(i64),
            Identifier(String),  // Um Variablen wie `x` zu repräsentieren
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

            Print(Box<ASTNode>),  // Füge dies hinzu, um die `print`-Anweisung zu unterstützen
        }

pub fn parse_assignment(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    println!("Starting to parse assignment, current token: {:?}", tokens.get(0));

    if let Some(Token::Var) = tokens.get(0).cloned() {
        tokens.remove(0);  // Entferne `var`

        if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
            tokens.remove(0);  // Entferne den Variablennamen

            if let Some(Token::Equal) = tokens.get(0).cloned() {
                tokens.remove(0);  // Entferne das Gleichheitszeichen `=`

                if let Some(Token::Number(value)) = tokens.get(0).cloned() {
                    tokens.remove(0);  // Entferne die Zahl

                    // Now check and remove the semicolon after the assignment
                    if let Some(Token::Semicolon) = tokens.get(0).cloned() {
                        tokens.remove(0);  // Entferne das Semikolon `;`
                        println!("Semicolon removed after assignment.");
                    } else {
                        println!("Error: Expected semicolon after assignment.");
                        return None;
                    }

                    // Return the assignment AST node
                    return Some(ASTNode::Assignment {
                        var_name: var_name.clone(),
                        value: Box::new(ASTNode::Number(value)),
                    });
                }
            }
        }
    }

    println!("Failed to parse assignment.");
    None
}

pub fn parse_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    println!("Starting parse_expression, current token: {:?}", tokens.get(0));

    // Check if it's an assignment expression (e.g., `x = x - 1`)
    if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
        println!("Detected Identifier for assignment: {}", var_name);
        tokens.remove(0);  // Remove the variable (e.g., `x`)

        // Check if the next token is an equal sign (`=`)
        if let Some(Token::Equal) = tokens.get(0).cloned() {
            println!("Detected equal sign `=` for assignment");
            tokens.remove(0);  // Remove the equal sign `=`

            // Try parsing the entire expression on the right-hand side (e.g., `x - 1`)
            if let Some(right_expr) = parse_binary_op(tokens) {
                println!("Parsed right-hand side of assignment: {:?}", right_expr);
                // Create an assignment node
                return Some(ASTNode::Assignment {
                    var_name,
                    value: Box::new(right_expr),
                });
            } else {
                println!("Error: Expected an expression after `=`");
                return None;
            }
        } else {
            println!("No equal sign found, returning the simple variable: {}", var_name);
            // If no equal sign, it's just a simple variable expression
            // If no equal sign, check for a binary operation (e.g., `x > 5`)
            if let Some(operator) = tokens.get(0).cloned() {
                match operator {
                    Token::GreaterThan | Token::LessThan | Token::GreaterEqual | Token::LessEqual | Token::Plus | Token::Minus => {
                        println!("Detected operator: {:?}", operator);
                        tokens.remove(0);  // Remove the operator

                        // Parse the right-hand side expression (e.g., `5` in `x > 5`)
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
                        println!("No binary operator found, returning the simple variable: {}", var_name);
                        return Some(ASTNode::Identifier(var_name));  // Just return the variable if no operator found
                    }
                }
            } else {
                println!("No operator found, returning the simple variable: {}", var_name);
                return Some(ASTNode::Identifier(var_name));
            }
        }

    }

    // Handle `print` statements (e.g., `print(x);`)
    if let Some(Token::Print) = tokens.get(0).cloned() {
        println!("Detected `print` statement");
        tokens.remove(0);  // Remove `print`

        // Expect an opening parenthesis `(` after `print`
        if let Some(Token::LeftParen) = tokens.get(0).cloned() {
            println!("Detected opening parenthesis `(` after `print`");
            tokens.remove(0);  // Remove `(`

            // Expect the expression inside the parentheses (e.g., the variable to print)
            if let Some(expression) = parse_primary_expression(tokens) {
                println!("Parsed expression inside `print`: {:?}", expression);

                // Expect the closing parenthesis `)`
                if let Some(Token::RightParen) = tokens.get(0).cloned() {
                    println!("Detected closing parenthesis `)` after expression");
                    tokens.remove(0);  // Remove `)`

                    // Now expect a semicolon `;` to end the statement
                    if let Some(Token::Semicolon) = tokens.get(0).cloned() {
                        println!("Detected semicolon `;` after `print` statement");
                        tokens.remove(0);  // Remove `;`
                        return Some(ASTNode::Print(Box::new(expression)));  // Return the `print` statement
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

    // If it's not a `print` statement, try parsing other expressions (e.g., binary expressions)
    if let Some(left) = parse_primary_expression(tokens) {
        println!("Parsed left-hand side expression: {:?}", left);

        // Check if there's an operator following the left-hand side (e.g., `+`, `-`, etc.)
        if let Some(operator) = tokens.get(0).cloned() {
            println!("Detected operator: {:?}", operator);
            tokens.remove(0);  // Remove the operator

            // Match supported operators and parse the right-hand side expression
            match operator {
                Token::GreaterThan | Token::LessThan | Token::GreaterEqual | Token::LessEqual | Token::Minus | Token::Plus => {
                    if let Some(right) = parse_primary_expression(tokens) {
                        println!("Parsed right-hand side expression after operator: {:?}", right);
                        tokens.remove(0);  // Remove the right-hand side expression token

                        return Some(ASTNode::BinaryOp {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        });
                    } else {
                        println!("Error: Expected right-hand side expression after operator");
                        return None;
                    }
                }
                _ => {
                    println!("Operator not recognized, returning the left-hand side as is");
                    return Some(left);
                }
            }
        } else {
            println!("No operator found, returning left-hand side expression as is");
            return Some(left);
        }
    }

    println!("No valid expression found, returning `None`");
    None
}

pub fn parse_primary_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    println!("Current token in parse_primary_expression: {:?}", tokens.get(0));

    // Case 1: Expression is wrapped in parentheses
    if let Some(Token::LeftParen) = tokens.get(0).cloned() {
        println!("Recognized opening paren `(`");
        tokens.remove(0);  // Remove `(`

        // Parse the expression inside the parentheses
        let expression = parse_expression(tokens);
        if let Some(Token::RightParen) = tokens.get(0).cloned() {
            println!("Recognized closing paren `)`");
            tokens.remove(0);  // Remove `)`
            return expression;
        } else {
            println!("Error: Expected closing paren `)`");
            return None;
        }
    }

    // Case 2: Expression without parentheses (e.g., an identifier or a number)
    if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
        println!("Recognized identifier: {}", var_name);
        tokens.remove(0);  // Remove identifier
        return Some(ASTNode::Identifier(var_name));
    }

    if let Some(Token::Number(value)) = tokens.get(0).cloned() {
        println!("Recognized number: {}", value);
        tokens.remove(0);  // Remove number
        return Some(ASTNode::Number(value));
    }

    println!("Error: No valid primary expression found");
    None
}

pub fn parse_if(tokens: &mut Vec<Token>) -> Option<ASTNode>  {
    if let Some(Token::If) = tokens.get(0).cloned() {
        println!("_____Found `if` keyword");
        tokens.remove(0);  // Remove `if`

        let condition = parse_expression(tokens)?;  // Parse the condition inside parentheses
        println!("_____Parsed condition: {:?}", condition);
        //tokens.remove(0);


        // Now expecting a `{` for the `then` block
        let next_token = tokens.get(0);  // Look at the next token without removing it
        println!("_____Next token after condition: {:?}", next_token);

        if let Some(Token::LeftBrace) = next_token {
            println!("_____Found opening brace `{{` for `then` block");
           // tokens.remove(0);  // Remove `{`

            // Parse the `then` block
            println!("Tokens: {:?}", tokens);
            let then_branch = parse_block(tokens)?;
            println!("_____Parsed `then` block: {:?}", then_branch);

            // Optionally parse the `else` block if present
            let else_branch = if let Some(Token::Else) = tokens.get(0) {
                println!("_____Found `else` keyword");
                tokens.remove(0);  // Remove `else`

                if let Some(Token::LeftBrace) = tokens.get(0) {
                    println!("_____Found opening brace `{{` for `else` block");
                    Some(Box::new(parse_block(tokens)?))  // Parse the `else` block
                } else {
                    println!("Error: `else` block must start with `{{`");
                    return None;
                }
            } else {
                println!("No `else` block found");
                None
            };

            return Some(ASTNode::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch,
            });
        } else {
            println!("Error: `then` block must start with `{{`. Found: {:?}", next_token);
            return None;
        }
    }
    None
}

//      if x > 5 { print(x); } else { print(0); }
        //      while x > 5 { print(x); }
pub fn parse_while(tokens: &mut Vec<Token>) -> Option<ASTNode> {
            if let Some(Token::While) = tokens.get(0).cloned() {
                println!("_____Found `while` keyword");
                tokens.remove(0);  // Entferne `while`

                // Parse die Bedingung der Schleife
                let condition = parse_expression(tokens)?;  // Die Bedingung sollte z.B. `x > 5` sein
                println!("_____Parsed condition: {:?}", condition);
                // Erwarte die öffnende geschweifte Klammer `{` für den Schleifenbody
                let body = parse_block(tokens)?;  // Parsen des Schleifenbodys, z.B. `{ print(x); }`
                println!("_____Parsed body: {:?}", body);
                // Gib den AST-Knoten für die `while`-Schleife zurück
                return Some(ASTNode::While {
                    condition: Box::new(condition),
                    body: Box::new(body),
                });
            }
            None
        }

pub fn parse_block(tokens: &mut Vec<Token>) -> Option<ASTNode> {
    println!("Parsing block, current token: {:?}", tokens.get(0));

    if let Some(Token::LeftBrace) = tokens.get(0) {
        tokens.remove(0);  // Remove the `{`
        let mut statements = Vec::new();

        while let Some(token) = tokens.get(0) {
            println!("Current token in block: {:?}", token);

            // Check if we've reached the closing brace `}`
            if let Token::RightBrace = token {
                tokens.remove(0);  // Remove the `}`
                println!("Found closing brace `}}`");
                return Some(ASTNode::Block(statements));  // Return the parsed block
            }

            // Parse the next expression or statement within the block
            if let Some(statement) = parse_expression(tokens) {
                statements.push(statement);

                // After successfully parsing a statement, expect a semicolon
                if let Some(Token::Semicolon) = tokens.get(0) {
                    println!("Found semicolon after statement, removing it.");
                    tokens.remove(0);  // Remove the semicolon `;`

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
    println!("Parsed left-hand side: {:?}", left);

    if let Some(operator) = tokens.get(0).cloned() {
        match operator {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::GreaterThan | Token::LessThan => {
                println!("Found binary operator: {:?}", operator);
                tokens.remove(0);  // Remove the operator

                let right = parse_primary_expression(tokens)?;
                println!("Parsed right-hand side: {:?}", right);

                return Some(ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                });
            }
            _ => {
                println!("No valid operator found, returning left-hand side");
                return Some(left);
            }
        }
    }
    None
}

mod tests {
            use super::*;
            use crate::lexer::tokenize;

            #[test]
            fn test_parse_assignment() {
                let input = "var x = 10;";
                let mut tokens = tokenize(input);
                let ast = parse_assignment(&mut tokens);
                assert!(ast.is_some());
            }

            #[test]
            fn test_parse_expression() {
                let input = "x = x - 1;";
                let mut tokens = tokenize(input);
                let ast = parse_expression(&mut tokens);
                assert!(ast.is_some());
            }
            #[test]

            fn test_parse_if_else() {
                let input = "if x > 5 { print(x); } else { print(0); }";
                let mut tokens = tokenize(input);
                let ast = parse_if(&mut tokens);
                assert!(ast.is_some());

                if let Some(ASTNode::If { condition, then_branch, else_branch }) = ast {
                    // Ensure the condition is correctly parsed
                    if let ASTNode::BinaryOp { left, operator, right } = *condition {
                        assert!(matches!(*left, ASTNode::Identifier(_)));
                        assert_eq!(operator, Token::GreaterThan);  // Dereference operator here
                        assert!(matches!(*right, ASTNode::Number(_)));
                    }

                    // Ensure the `then` branch is a block containing a `print` statement
                    if let ASTNode::Block(statements) = *then_branch {
                        if let ASTNode::Print(expr) = &statements[0] {
                            assert!(matches!(**expr, ASTNode::Identifier(_)));
                        }
                    }

                    // Ensure the `else` branch is a block containing a `print` statement
                    if let Some(ASTNode::Block(statements)) = else_branch.as_deref() {
                        if let ASTNode::Print(expr) = &statements[0] {
                            assert!(matches!(**expr, ASTNode::Number(_)));
                        }
                    }
                }
            }

    #[test]
            fn test_parse_while_loop() {
                let input = "while x > 5 { print(x); x = x - 1; }";
                let mut tokens = tokenize(input);
                let ast = parse_while(&mut tokens);
                assert!(ast.is_some());
            }

    #[test]
    fn test_simple_if_parsing() {
        let input = "if x > 5 { print(x); }";
        let mut tokens = tokenize(input);
        let ast = parse_if(&mut tokens);

        // Ensure that the AST is generated
        assert!(ast.is_some(), "Expected some AST, but got None");

        if let Some(ASTNode::If { condition, then_branch, .. }) = ast {
            // Check if the condition is parsed as `x > 5`
            match *condition {
                ASTNode::BinaryOp { ref left, ref operator, ref right } => {
                    if let ASTNode::Identifier(ref name) = **left {
                        assert_eq!(name, "x", "Expected left operand to be 'x'");
                    } else {
                        panic!("Expected Identifier for left operand");
                    }

                    assert_eq!(*operator, Token::GreaterThan, "Expected '>' operator");

                    if let ASTNode::Number(value) = **right {
                        assert_eq!(value, 5, "Expected right operand to be '5'");
                    } else {
                        panic!("Expected Number for right operand");
                    }
                }
                _ => panic!("Expected BinaryOp in condition"),
            }

            // Check if the then branch contains the print statement `print(x)`
            match *then_branch {
                ASTNode::Block(ref statements) => {
                    if let ASTNode::Print(ref expr) = statements[0] {
                        if let ASTNode::Identifier(ref name) = **expr {
                            assert_eq!(name, "x", "Expected 'x' inside print statement");
                        } else {
                            panic!("Expected Identifier 'x' in print statement");
                        }
                    } else {
                        panic!("Expected Print statement in 'then' block");
                    }
                }
                _ => panic!("Expected Block in 'then' branch"),
            }
        } else {
            panic!("AST for 'if' statement not parsed correctly.");
        }
    }

    #[test]
            fn test_while_loop_parsing() {
                let input = "while x > 5 { print(x); x = x - 1; }";
                let mut tokens = tokenize(input);
                let ast = parse_while(&mut tokens);

                // Ensure the AST is successfully created
                assert!(ast.is_some());

                if let Some(ASTNode::While { condition, body }) = ast {
                    // Check the condition is correctly parsed as `x > 5`
                    match *condition {
                        ASTNode::BinaryOp { ref left, ref operator, ref right } => {
                            match **left {
                                ASTNode::Identifier(ref name) => assert_eq!(name, "x"),
                                _ => panic!("Expected Identifier for left operand in condition"),
                            }
                            assert_eq!(*operator, Token::GreaterThan); // Dereference the operator
                            match **right {
                                ASTNode::Number(value) => assert_eq!(value, 5),
                                _ => panic!("Expected Number 5 for right operand in condition"),
                            }
                        }
                        _ => panic!("Expected BinaryOp in condition"),
                    }

                    // Check the body contains both `print(x)` and `x = x - 1`
                    match *body {
                        ASTNode::Block(ref statements) => {
                            assert_eq!(statements.len(), 2);

                            // Check the first statement is `print(x)`
                            if let ASTNode::Print(ref expr) = statements[0] {
                                match **expr {
                                    ASTNode::Identifier(ref name) => assert_eq!(name, "x"),
                                    _ => panic!("Expected Identifier 'x' in print statement"),
                                }
                            } else {
                                panic!("Expected Print statement in while body");
                            }

                            // Check the second statement is `x = x - 1`
                            if let ASTNode::Assignment { ref var_name, ref value } = statements[1] {
                                assert_eq!(var_name, "x");
                                if let ASTNode::BinaryOp { ref left, ref operator, ref right } = **value {
                                    match **left {
                                        ASTNode::Identifier(ref name) => assert_eq!(name, "x"),
                                        _ => panic!("Expected Identifier 'x' in assignment"),
                                    }
                                    assert_eq!(*operator, Token::Minus); // Dereference the operator
                                    match **right {
                                        ASTNode::Number(value) => assert_eq!(value, 1),
                                        _ => panic!("Expected Number 1 in assignment"),
                                    }
                                } else {
                                    panic!("Expected BinaryOp in assignment");
                                }
                            } else {
                                panic!("Expected Assignment in while body");
                            }
                        }
                        _ => panic!("Expected Block in while body"),
                    }
                } else {
                    panic!("AST for 'while' loop not parsed correctly.");
                }
            }
        }

