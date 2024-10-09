#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::lexer::tokenize;
    use crate::parser::{parse_expression, parse_if, parse_while};
    use crate::interpreter::interpret;

    #[test]
    fn test_variable_assignment() {
        let input = "var x = 10;";
        let mut tokens = tokenize(input);
        let mut env = HashMap::new();
        if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 10);
    }

    #[test]
    fn test_variable_assignment_with_expression() {
        let input = "x = x - 1;";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10); // Predefine x with value 10
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 9); // Expect x to be 9 after evaluation
    }

    #[test]
    fn test_print_statement() {
        let input = "print(x);";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10); // Predefine x with value 10
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_expression(&mut tokens) {
            interpret(ast, &mut env);
        }
        // You can manually verify printed output for now, or redirect stdout to capture it in more advanced cases.
    }

    #[test]
    fn test_while_loop() {
        let input = "while x > 7 { print(x); x = x - 1; }";
        let mut env = HashMap::new();
        env.insert("x".to_string(), 10); // Predefine x with value 10
        let mut tokens = tokenize(input);
        if let Some(ast) = parse_while(&mut tokens) {
            interpret(ast, &mut env);
        }
        assert_eq!(*env.get("x").unwrap(), 7); // Expect x to be 7 after the loop
    }
}
