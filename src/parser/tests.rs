#[cfg(test)]
mod parser_tests {
    use crate::parser::{ast::Expr, error::ParserError, pratt::Parser};

    use crate::lexer::{token::Token, tokenizer::Lexer};

    fn parse(input: &str) -> Result<Expr, ParserError> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let mut exprs = parser.parse()?;
        Ok(exprs.remove(0))
    }

    #[test]
    fn test_basic_addition() {
        let ast = parse("1 + 2").unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(1.0)),
                op: Token::Plus,
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn test_right_associative_exponentiation() {
        let ast = parse("2 ^ 3 ^ 2").unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                op: Token::Caret,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(3.0)),
                    op: Token::Caret,
                    right: Box::new(Expr::Number(2.0)),
                })
            }
        );
    }

    #[test]
    fn test_precedence_multiplication() {
        let ast = parse("1 + 2 * 3").unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(1.0)),
                op: Token::Plus,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(2.0)),
                    op: Token::Star,
                    right: Box::new(Expr::Number(3.0)),
                })
            }
        );
    }

    #[test]
    fn test_parentheses() {
        let ast = parse("(1 + 2) * 3").unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(1.0)),
                    op: Token::Plus,
                    right: Box::new(Expr::Number(2.0)),
                }),
                op: Token::Star,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn test_unary_minus() {
        let ast = parse("-x").unwrap();
        assert_eq!(
            ast,
            Expr::Unary {
                op: Token::Minus,
                expr: Box::new(Expr::Identifier("x".into())),
            }
        );
    }

    #[test]
    fn test_unary_plus() {
        let ast = parse("+x").unwrap();
        assert_eq!(
            ast,
            Expr::Unary {
                op: Token::Plus,
                expr: Box::new(Expr::Identifier("x".into())),
            }
        );
    }

    #[test]
    fn test_implicit_multiplication_number_identifier() {
        let ast = parse("2x").unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                op: Token::Star,
                right: Box::new(Expr::Identifier("x".into())),
            }
        );
    }

    #[test]
    fn test_implicit_multiplication_parentheses() {
        let ast = parse("(x+1)(y+2)").unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Identifier("x".into())),
                    op: Token::Plus,
                    right: Box::new(Expr::Number(1.0)),
                }),
                op: Token::Star,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Identifier("y".into())),
                    op: Token::Plus,
                    right: Box::new(Expr::Number(2.0)),
                }),
            }
        );
    }

    #[test]
    fn test_function_call_no_args() {
        let ast = parse("foo()").unwrap();
        assert_eq!(
            ast,
            Expr::Call {
                function: Box::new(Expr::Identifier("foo".into())),
                args: vec![],
            }
        );
    }

    #[test]
    fn test_function_call_with_args() {
        let ast = parse("max(1, x, 3)").unwrap();
        assert_eq!(
            ast,
            Expr::Call {
                function: Box::new(Expr::Identifier("max".into())),
                args: vec![
                    Expr::Number(1.0),
                    Expr::Identifier("x".into()),
                    Expr::Number(3.0),
                ],
            }
        );
    }

    #[test]
    fn test_function_definition() {
        let ast = parse("f(x,y) = x + y").unwrap();
        assert_eq!(
            ast,
            Expr::Function {
                name: "f".into(),
                args: vec!["x".into(), "y".into()],
                body: Box::new(Expr::Binary {
                    left: Box::new(Expr::Identifier("x".into())),
                    op: Token::Plus,
                    right: Box::new(Expr::Identifier("y".into())),
                }),
            }
        );
    }

    #[test]
    fn test_assignment() {
        let ast = parse("x = 5").unwrap();
        assert_eq!(
            ast,
            Expr::Assignment {
                target: "x".into(),
                value: Box::new(Expr::Number(5.0)),
            }
        );
    }

    #[test]
    fn test_invalid_assignment_target() {
        let err = parse("5 = x").unwrap_err();
        match err {
            ParserError::InvalidAssignment(_, _, _) => {}
            _ => panic!("Expected InvalidAssignment"),
        }
    }

    #[test]
    fn test_comparison() {
        let ast = parse("x < 10").unwrap();
        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Identifier("x".into())),
                op: Token::Less,
                right: Box::new(Expr::Number(10.0)),
            }
        );
    }

    #[test]
    fn test_unexpected_token_error() {
        let err = parse(")").unwrap_err();
        match err {
            ParserError::UnexpectedToken(Token::RParen, _) => {}
            _ => panic!("Expected UnexpectedToken"),
        }
    }

    #[test]
    fn test_missing_parenthesis() {
        let err = parse("(1 + 2").unwrap_err();
        match err {
            ParserError::UnexpectedToken(Token::Eof, _) => {}
            _ => panic!("Expected UnexpectedToken for missing ')'"),
        }
    }
}
