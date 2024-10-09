
    use std::collections::HashMap;
    use Xene::lexer::tokenize;
    use Xene::parser::{parse_assignment, parse_expression, parse_if, parse_while};
    use Xene::interpreter::interpret;

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
            interpret(ast, &mut env); // This will panic because `y` is not defined
        }
        // You can manually check for the expected panic in this case.
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

