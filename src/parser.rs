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
            if let Some(Token::Var) = tokens.get(0).cloned() {
                tokens.remove(0);  // Entferne `var`
                if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
                    tokens.remove(0);  // Entferne den Variablennamen
                    if let Some(Token::Equal) = tokens.get(0).cloned() {
                        tokens.remove(0);  // Entferne das Gleichheitszeichen `=`
                        if let Some(Token::Number(value)) = tokens.get(0).cloned() {
                            tokens.remove(0);  // Entferne die Zahl
                            return Some(ASTNode::Assignment {
                                var_name: var_name.clone(),
                                value: Box::new(ASTNode::Number(value)),
                            });
                        }
                    }
                }
            }
            None
        }

        pub fn parse_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
            println!("Current token in parse_expression: {:?}", tokens.get(0));

            // Überprüfen, ob es sich um eine Zuweisung handelt (z.B. `x = x - 1`)
            if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
                println!("Erkannte Identifier für Zuweisung: {}", var_name);
                tokens.remove(0);  // Entferne die Variable (z.B. `x`)

                // Überprüfen, ob ein Gleichheitszeichen folgt (z.B. `=`)
                if let Some(Token::Equal) = tokens.get(0).cloned() {
                    println!("Erkannte Gleichheitszeichen `=`");
                    tokens.remove(0);  // Entferne das Gleichheitszeichen `=`

                    // Versuche, den gesamten Ausdruck auf der rechten Seite zu parsen (z.B. `x - 1`)
                    if let Some(right_expr) = parse_binary_op(tokens) {
                        println!("Parsed rechter Ausdruck für Zuweisung: {:?}", right_expr);
                        // Erstelle einen Zuweisungsknoten
                        return Some(ASTNode::Assignment {
                            var_name,
                            value: Box::new(right_expr),
                        });
                    } else {
                        println!("Fehler: Erwarte rechten Ausdruck nach `=`");
                        return None;
                    }
                } else {
                    println!("Kein Gleichheitszeichen gefunden, Rückgabe der einfachen Variable: {}", var_name);
                    // Falls kein Gleichheitszeichen vorhanden ist, handelt es sich möglicherweise um eine einfache Variable
                    return Some(ASTNode::Identifier(var_name));
                }
            }

            // Handhabung für `print`
            if let Some(Token::Print) = tokens.get(0) {
                tokens.remove(0);  // Entferne `print`

                // Erwarte die öffnende Klammer `(`
                if let Some(Token::LeftParen) = tokens.get(0) {
                    tokens.remove(0);  // Entferne `(`

                    // Erwarte den Ausdruck innerhalb der Klammern (z.B. eine Variable)
                    if let Some(expression) = parse_primary_expression(tokens) {
                        // Erwarte die schließende Klammer `)`
                        if let Some(Token::RightParen) = tokens.get(0) {
                            tokens.remove(0);  // Entferne `)`

                            // Jetzt sollte ein Semikolon `;` folgen
                            if let Some(Token::Semicolon) = tokens.get(0) {
                                tokens.remove(0);  // Entferne das Semikolon
                                return Some(ASTNode::Print(Box::new(expression)));  // Gib die `print`-Anweisung zurück
                            } else {
                                println!("Fehler: Fehlendes Semikolon nach `print`");
                                return None;
                            }
                        } else {
                            println!("Fehler: Fehlende schließende Klammer `)` nach Ausdruck");
                            return None;
                        }
                    } else {
                        println!("Fehler: Ungültiger Ausdruck in `print`");
                        return None;
                    }
                } else {
                    println!("Fehler: Fehlende öffnende Klammer `(` nach `print`");
                    return None;
                }
            }

            // Falls es keine `print`-Anweisung ist, versuche eine andere Expression
            if let Some(left) = parse_primary_expression(tokens) {
                println!("Parsed linker Ausdruck: {:?}", left);
                if let Some(operator) = tokens.get(0).cloned() {
                    println!("Erkannter Operator: {:?}", operator);
                    tokens.remove(0);  // Entferne den Operator
                    match operator {
                        Token::GreaterThan | Token::LessThan | Token::GreaterEqual | Token::LessEqual | Token::Minus | Token::Plus => {
                            if let Some(right) = parse_primary_expression(tokens) {
                                println!("Parsed rechter Ausdruck nach Operator: {:?}", right);
                                return Some(ASTNode::BinaryOp {
                                    left: Box::new(left),
                                    operator,
                                    right: Box::new(right),
                                });
                            } else {
                                println!("Fehler: Erwarte rechten Ausdruck nach Operator");
                                return None;
                            }
                        }
                        _ => {
                            return Some(left);
                        }
                    }
                }
            }

            None
        }

        pub fn parse_primary_expression(tokens: &mut Vec<Token>) -> Option<ASTNode> {
            println!("Current token in parse_primary_expression: {:?}", tokens.get(0));

            // Prüfe auf Zahlen oder Variablen als primäre Ausdrücke
            if let Some(Token::Number(value)) = tokens.get(0).cloned() {
                println!("Erkannte Zahl: {}", value);
                tokens.remove(0);
                return Some(ASTNode::Number(value));
            }

            if let Some(Token::Identifier(var_name)) = tokens.get(0).cloned() {
                println!("Erkannte Variable: {}", var_name);
                tokens.remove(0);
                return Some(ASTNode::Identifier(var_name));
            }

            println!("Fehler: Kein gültiger primärer Ausdruck gefunden");
            None
        }

        pub fn parse_if(tokens: &mut Vec<Token>) -> Option<ASTNode> {
            if let Some(Token::If) = tokens.get(0).cloned() {
                tokens.remove(0);  // Entferne `if`

                // Hier wird eventuell nach einer öffnenden Klammer gesucht, obwohl sie nicht da ist
                if let Some(Token::LeftParen) = tokens.get(0) {
                    tokens.remove(0);  // Entferne `(`
                }

                let condition = parse_expression(tokens)?;  // Parse die Bedingung

                if let Some(Token::RightParen) = tokens.get(0) {
                    tokens.remove(0);  // Entferne `)`
                }

                // Parse den Block nach dem If
                if let Some(Token::LeftBrace) = tokens.get(0) {
                    tokens.remove(0);  // Entferne `{`
                    let then_branch = parse_block(tokens)?;  // Parse den Block

                    // Optional: Überprüfen, ob es einen `else`-Zweig gibt
                    let else_branch = if let Some(Token::Else) = tokens.get(0) {
                        tokens.remove(0);  // Entferne `else`
                        Some(Box::new(parse_block(tokens)?))  // Parse den `else`-Block
                    } else {
                        None
                    };

                    return Some(ASTNode::If {
                        condition: Box::new(condition),
                        then_branch: Box::new(then_branch),
                        else_branch,
                    });
                } else {
                    println!("Fehler: `then`-Block beginnt nicht mit `{{`");
                    return None;
                }
            }
            None
        }

        //      if x > 5 { print(x); } else { print(0); }
        //      while x > 5 { print(x); }
        pub fn parse_while(tokens: &mut Vec<Token>) -> Option<ASTNode> {
            if let Some(Token::While) = tokens.get(0).cloned() {
                tokens.remove(0);  // Entferne `while`

                // Parse die Bedingung der Schleife
                let condition = parse_expression(tokens)?;  // Die Bedingung sollte z.B. `x > 5` sein

                // Erwarte die öffnende geschweifte Klammer `{` für den Schleifenbody
                let body = parse_block(tokens)?;  // Parsen des Schleifenbodys, z.B. `{ print(x); }`

                // Gib den AST-Knoten für die `while`-Schleife zurück
                return Some(ASTNode::While {
                    condition: Box::new(condition),
                    body: Box::new(body),
                });
            }
            None
        }

        pub fn parse_block(tokens: &mut Vec<Token>) -> Option<ASTNode> {
            // Überprüfen, ob der Block mit `{` beginnt
            if let Some(Token::LeftBrace) = tokens.get(0) {
                tokens.remove(0);  // Entferne die `{`

                let mut statements = Vec::new();

                // Parsen der Anweisungen innerhalb des Blocks
                while let Some(token) = tokens.get(0) {
                    println!("Current token in parse_block: {:?}", token);  // Debugging-Ausgabe

                    // Wenn wir auf `}` stoßen, wissen wir, dass der Block endet
                    if let Token::RightBrace = token {
                        tokens.remove(0);  // Entferne die `}`
                        println!("End of block detected.");  // Debugging
                        return Some(ASTNode::Block(statements));  // Gib den Block zurück
                    }

                    // Versuche, den nächsten Ausdruck oder die nächste Anweisung zu parsen
                    if let Some(statement) = parse_expression(tokens) {
                        statements.push(statement);
                    } else {
                        // Fehler: Ungültige Anweisung innerhalb des Blocks
                        println!("Fehler beim Parsen des Blocks.");
                        return None;
                    }
                }

                // Falls die Schleife endet, ohne ein `}` zu finden, liegt ein Fehler vor
                println!("Fehler: Kein `brace` gefunden, Block wurde nicht richtig geschlossen.");
                return None;
            }

            println!("Fehler: Block beginnt nicht mit `brace`.");
            None
        }

        pub fn parse_binary_op(tokens: &mut Vec<Token>) -> Option<ASTNode> {
            if let Some(left) = parse_primary_expression(tokens) {
                if let Some(operator) = tokens.get(0).cloned() {
                    tokens.remove(0);  // Entferne den Operator

                    // Parsen des rechten Ausdrucks
                    if let Some(right) = parse_primary_expression(tokens) {
                        return Some(ASTNode::BinaryOp {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        });
                    } else {
                        println!("Fehler: Erwarte rechten Ausdruck nach Operator");
                        return None;
                    }
                } else {
                    return Some(left);  // Falls kein Operator folgt, gib einfach den linken Ausdruck zurück
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
                let input = "if ( x > 5 ){ print(x); } else { print(0); }";
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
            fn test_if_statement_parsing() {
                let input = "if x > 5 { print(x); } else { print(0); }";
                let mut tokens = tokenize(input);
                let ast = parse_if(&mut tokens);

                // Ensure the AST is successfully created
                assert!(ast.is_some());

                if let Some(ASTNode::If { condition, then_branch, else_branch }) = ast {
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

                    // Check the `then` branch contains the print statement `print(x)`
                    match *then_branch {
                        ASTNode::Block(ref statements) => {
                            assert_eq!(statements.len(), 1);
                            if let ASTNode::Print(ref expr) = statements[0] {
                                match **expr {
                                    ASTNode::Identifier(ref name) => assert_eq!(name, "x"),
                                    _ => panic!("Expected Identifier 'x' in print statement"),
                                }
                            } else {
                                panic!("Expected Print statement in 'then' block");
                            }
                        }
                        _ => panic!("Expected Block in 'then' branch"),
                    }

                    // Check the `else` branch contains the print statement `print(0)`
                    if let Some(ASTNode::Block(ref statements)) = else_branch.as_deref() {
                        assert_eq!(statements.len(), 1);
                        if let ASTNode::Print(ref expr) = statements[0] {
                            match **expr {
                                ASTNode::Number(value) => assert_eq!(value, 0),
                                _ => panic!("Expected Number 0 in print statement of 'else' branch"),
                            }
                        } else {
                            panic!("Expected Print statement in 'else' block");
                        }
                    } else {
                        panic!("Expected 'else' branch in AST");
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

