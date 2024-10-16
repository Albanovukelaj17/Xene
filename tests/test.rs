
    use std::collections::HashMap;
    use Xene::lexer::{tokenize, Token};
    use Xene::parser::{parse_assignment, parse_expression, parse_if, parse_while, ASTNode};
    use Xene::interpreter::interpret;



    //LEXER--------------



    #[test]
    fn test_tokenize_var_assignment() {
        let input = "var x = 10;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 6);  // var, x, =, 10, ; ,eof
    }

    #[test]
    fn test_tokenize_binary_expression() {
        let input = "x = x - 1;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 7);  // x, =, x, -, 1, ; , eof
    }

    #[test]
    fn test_tokenize_condition() {
        let input = "if x >= 10;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 6);  // if, x, >=, 10, ;, Eof

        assert_eq!(tokens[0], Token::If);
        assert!(matches!(tokens[1], Token::Identifier(_)));
        assert_eq!(tokens[2], Token::GreaterEqual);
        assert!(matches!(tokens[3], Token::Number(10)));
        assert_eq!(tokens[4], Token::Semicolon);
        assert_eq!(tokens[5], Token::Eof);
    }
    #[test]
    fn test_tokenize_multiple_statements() {
        let input = "var x = 5; x = x + 1;";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 12);  // var, x, =, 5, ;, x, =, x, +, 1, ;, Eof

        assert_eq!(tokens[0], Token::Var);
        assert!(matches!(tokens[1], Token::Identifier(_)));
        assert_eq!(tokens[2], Token::Equal);
        assert!(matches!(tokens[3], Token::Number(5)));
        assert_eq!(tokens[4], Token::Semicolon);

        assert!(matches!(tokens[5], Token::Identifier(_)));
        assert_eq!(tokens[6], Token::Equal);
        assert!(matches!(tokens[7], Token::Identifier(_)));
        assert_eq!(tokens[8], Token::Plus);
        assert!(matches!(tokens[9], Token::Number(1)));
        assert_eq!(tokens[10], Token::Semicolon);
    }

    #[test]
    fn test_tokenize_with_braces_and_parens() {
        let input = "if (x > 5) { print(x); }";
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), 14);  // if, (, x, >, 5, ), {, print, (, x, ), ;, }

        assert_eq!(tokens[0], Token::If);                      // if
        assert_eq!(tokens[1], Token::LeftParen);               // (
        assert!(matches!(tokens[2], Token::Identifier(_)));     // x
        assert_eq!(tokens[3], Token::GreaterThan);             // >
        assert!(matches!(tokens[4], Token::Number(5)));         // 5
        assert_eq!(tokens[5], Token::RightParen);              // )
        assert_eq!(tokens[6], Token::LeftBrace);               // {
        assert_eq!(tokens[7], Token::Print);                   // print
        assert_eq!(tokens[8], Token::LeftParen);               // (
        assert!(matches!(tokens[9], Token::Identifier(_)));     // x
        assert_eq!(tokens[10], Token::RightParen);             // )
        assert_eq!(tokens[11], Token::Semicolon);               //;
        assert_eq!(tokens[12], Token::RightBrace);             // }
        assert!(matches!(tokens[13], Token::Eof));
    }


   // PARSER-----------



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

//INTERPRETER---------


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




    //INTEGRATE TESTSSSSS----------

    #[test]
    fn test_variable_assignment() {
        let input = "var x = 10;";
        let mut tokens = tokenize(input);
        let mut env = HashMap::new();
        if let Some(ast) = parse_assignment(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 10);
    }

    #[test]
    fn test_variable_assignment_with_expression() {
        let input = "x = x - 1;";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10); // Initialize x to 10
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 9); // Expect x to be 9 after decrement
    }

    #[test]
    fn test_print_statement() {
        let input = "print(x);";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10); // Initialize x to 10
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env);
        }
        // You would need to manually verify the printed output for now.
    }

    #[test]
    fn test_binary_operation() {
        let input = "var x = 5 + 3;";
        let mut tokens = tokenize(input);
        let mut env = HashMap::new();
        if let Some(ast) = parse_assignment(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 8); // Expect x to be 8 (5 + 3)
    }

    #[test]
    fn test_if_else_statement() {
        let input = "if x > 5 { print(x); } else { print(0); }";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 6); // Initialize x to 6
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_if(&mut tokens) {
            interpret(ast, &mut env);
        }
        // Manual print verification is needed for the correct output.
    }

    #[test]
    fn test_while_loop() {
        let input = "while x > 5 { print(x); x = x - 1; }";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10); // Initialize x to 10
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_while(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 5); // Expect x to be 5 after loop
    }

    #[test]
    fn test_invalid_variable_access() {
        let input = "print(y);";
        let mut env = HashMap::new();
        let mut tokens = tokenize(input);

        if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env);
        }

        // In this case, we expect that 'y' is undefined and should have printed an error.
        // Check if the error message is printed (you can mock or capture output if necessary).
    }


    #[test]
    fn test_invalid_expression() {
        let input = "x = x + ;";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10); // Initialize x to 10
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env); // This will fail because the expression is invalid
        }
        // You can manually check for the expected failure in this case.
    }

    #[test]
    fn test_if_else_condition_false() {
        let input = "if x > 5 { print(x); } else { print(0); }";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 4); // Initialize x to 4
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_if(&mut tokens) {
            interpret(ast, &mut env);
        }
        // Manual verification for the printed value (0)
    }

    #[test]
    fn test_multiple_assignments_and_loop() {
        let input = "
        var a = 5;
        var b = 3;
        var sum = a + b;
        while sum > 0 {
            sum = sum - 2;
        }";

        let mut env = HashMap::new();
        let mut tokens = tokenize(input);

        // Parse and execute multiple statements
        while !tokens.is_empty() {
            if let Some(ast) = parse_assignment(&mut tokens) {
                interpret(ast, &mut env);
            } else if let Some(ast) = parse_while(&mut tokens) {
                interpret(ast, &mut env);
            } else {
                break; // Stop if there are no more valid tokens
            }
        }

        // After execution, `sum` should be 0, as it decreases by 2 in the loop until it reaches 0
        assert_eq!(*env.get("sum").unwrap(), 0);
    }
    #[test]
    fn test_nested_if_else_in_loop() {
        let input = "
        var x = 10;
        while x > 0 {
            if x % 2 == 0 {
                print(x);  // Should print only even numbers
            } else {
                print(-1);  // Print -1 for odd numbers
            }
            x = x - 1;
        }";

        let mut env = HashMap::new();
        env.insert("x".to_string(), 10);
        let mut tokens = tokenize(input);

        // Parse and execute
        if let Some(ast) = parse_while(&mut tokens) {
            interpret(ast, &mut env);
        }

        // After the loop, `x` should be 0
        assert_eq!(*env.get("x").unwrap(), 0);
    }
    #[test]
    fn test_complex_conditionals() {
        let input = "
        var x = 10;
        var y = 5;
        if (x > y) {
            x = x + y;
            print(x);  // Should print 15
        } else {
            y = y - 1;
            print(y);  // Should not print, as x > y is true
        }";

        let mut env = HashMap::new();
        let mut tokens = tokenize(input);

        // Parse and execute
        while !tokens.is_empty() {
            if let Some(ast) = parse_assignment(&mut tokens) {
                interpret(ast, &mut env);
            } else if let Some(ast) = parse_if(&mut tokens) {
                interpret(ast, &mut env);
            } else {
                break;
            }
        }

        // After execution, `x` should be 15, `y` should remain 5
        assert_eq!(*env.get("x").unwrap(), 15);
        assert_eq!(*env.get("y").unwrap(), 5);
    }
    #[test]
    fn test_complex_expressions_with_loop_and_condition() {
        let input = "
        var total = 0;
        var limit = 10;
        while limit > 0 {
            if limit % 3 == 0 {
                total = total + limit;
            } else {
                total = total - 1;
            }
            limit = limit - 1;
        }";

        let mut env = HashMap::new();
        let mut tokens = tokenize(input);

        // Parse and execute
        while !tokens.is_empty() {
            if let Some(ast) = parse_assignment(&mut tokens) {
                interpret(ast, &mut env);
            } else if let Some(ast) = parse_while(&mut tokens) {
                interpret(ast, &mut env);
            } else {
                break;
            }
        }

        // After execution, `total` should accumulate values based on the condition
        assert_eq!(*env.get("total").unwrap(), 8);  // Example expected result
    }
    #[test]
    fn test_multiple_loops_and_conditionals() {
        let input = "
        var a = 5;
        var b = 10;
        var c = 0;

        while a < b {
            if a % 2 == 0 {
                c = c + 2;
            } else {
                c = c + 1;
            }
            a = a + 1;
        }

        if c >= 10 {
            print(c);  // Expected to print 10 or more
        }";

        let mut env = HashMap::new();
        let mut tokens = tokenize(input);

        // Parse and execute
        while !tokens.is_empty() {
            if let Some(ast) = parse_assignment(&mut tokens) {
                interpret(ast, &mut env);
            } else if let Some(ast) = parse_while(&mut tokens) {
                interpret(ast, &mut env);
            } else if let Some(ast) = parse_if(&mut tokens) {
                interpret(ast, &mut env);
            } else {
                break;
            }
        }

        // After execution, `c` should be calculated based on loop and conditional
        assert_eq!(*env.get("c").unwrap(), 10);  // Example expected result
    }
    #[test]
    fn test_nested_loops() {
        let input = "
        var total = 0;
        var i = 0;
        while i < 5 {
            var j = 0;
            while j < 5 {
                total = total + 1;
                j = j + 1;
            }
            i = i + 1;
        }";

        let mut env = HashMap::new();
        let mut tokens = tokenize(input);

        // Parse and execute
        while !tokens.is_empty() {
            if let Some(ast) = parse_assignment(&mut tokens) {
                interpret(ast, &mut env);
            } else if let Some(ast) = parse_while(&mut tokens) {
                interpret(ast, &mut env);
            } else {
                break;
            }
        }

        // After execution, `total` should be 25 (5 * 5)
        assert_eq!(*env.get("total").unwrap(), 25);
    }
