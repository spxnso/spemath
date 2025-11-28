#[cfg(test)]
mod lexer_tests {
    use crate::lexer::{error::LexerError, token::{SpannedToken, Token}, tokenizer::Lexer};

    fn filter_tokens(tokens: Vec<SpannedToken>) -> Vec<Token> {
        tokens
            .into_iter()
            .map(|spanned| spanned.value)
            .filter(|t| *t != Token::Whitespace)
            .collect()
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("12 3.45 6.7 .89 1.23e4 5.6E-2");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            filter_tokens(tokens),
            vec![
                Token::Number(12.0),
                Token::Number(3.45),
                Token::Number(6.7),
                Token::Number(0.89),
                Token::Number(12300.0),
                Token::Number(0.056),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("x foo bar1 _baz");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            filter_tokens(tokens),
            vec![
                Token::Identifier("x".into()),
                Token::Identifier("foo".into()),
                Token::Identifier("bar1".into()),
                Token::Identifier("_baz".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("+ - * / % ^ = == != < <= > >= !");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            filter_tokens(tokens),
            vec![
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Slash,
                Token::Percent,
                Token::Caret,
                Token::Equal,
                Token::EqualEqual,
                Token::ExclamationEqual,
                Token::Less,
                Token::LessEqual,
                Token::Greater,
                Token::GreaterEqual,
                Token::Exclamation,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_parentheses_brackets_braces() {
        let mut lexer = Lexer::new("( ) [ ] { } , ;");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            filter_tokens(tokens),
            vec![
                Token::LParen,
                Token::RParen,
                Token::LBracket,
                Token::RBracket,
                Token::LBrace,
                Token::RBrace,
                Token::Comma,
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_whitespace_and_newlines() {
        let mut lexer = Lexer::new("x \n y\tz");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            filter_tokens(tokens),
            vec![
                Token::Identifier("x".into()),
                Token::Newline,
                Token::Identifier("y".into()),
                Token::Identifier("z".into()),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_invalid_number() {
        let mut lexer = Lexer::new("12.3.4");
        let err = lexer.tokenize().unwrap_err();
        assert_eq!(err.len(), 1);
        match &err[0] {
            LexerError::InvalidNumberFormat(s, _, _) => {
                println!("Invalid number format: {}", s);
                assert_eq!(s, "12.3.4");
            }
            _ => panic!("Expected InvalidNumberFormat"),
        }
    }

    #[test]
    fn test_unexpected_character() {
        let mut lexer = Lexer::new("@");
        let err = lexer.tokenize().unwrap_err();
        assert_eq!(err.len(), 1);
        match &err[0] {
            LexerError::UnexpectedCharacter(c, _, _) => assert_eq!(*c, '@'),
            _ => panic!("Expected UnexpectedCharacter"),
        }
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("// this is a comment\n42 /* multi\nline */ 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            filter_tokens(tokens),
            vec![
                Token::Newline,
                Token::Number(42.0),
                Token::Number(3.0),
                Token::Eof,
            ]
        );
    }
}
