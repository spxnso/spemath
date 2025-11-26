use crate::lexer::{error::LexerError, token::Token};
use log::{debug, warn};

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            chars: input.chars(),
            current_char: None,
            pos: 0,
            line: 1,
            column: 1,
        };
        lexer.current_char = lexer.chars.next();
        lexer
    }

    pub fn advance(&mut self) {
        debug!(
            "Advancing from char: {:?}, pos: ({}, {}, {})",
            self.current_char, self.line, self.column, self.pos
        );
        if let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            }
        }

        self.current_char = self.chars.next();
        self.pos += 1;
        self.column += 1;
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn whitespace(&mut self, tokens: &mut Vec<Token>) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                if c == '\n' {
                    self.push_token(tokens, Token::Newline);
                } else if c == ' ' || c == '\t' {
                    self.push_token(tokens, Token::Whitespace);
                } else {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let mut id_str = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                id_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::Identifier(id_str)
    }

    fn number(&mut self) -> Result<Token, LexerError> {
        let mut num_str = String::new();

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '-' || c == '+' {
                num_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        num_str
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| LexerError::InvalidNumberFormat(num_str, self.line, self.column))
    }

    fn push_token(&mut self, tokens: &mut Vec<Token>, token: Token) {
        tokens.push(token);
        self.advance();
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Vec<LexerError>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while let Some(c) = self.current_char {
            debug!(
                "Current char: {:?}, pos: ({}, {}, {})",
                c, self.line, self.column, self.pos
            );
            match c {
                '0'..='9' | '.' => match self.number() {
                    Ok(token) => {
                        debug!("Parsed number token: {:?}", token);
                        tokens.push(token)
                    }
                    Err(err) => {
                        warn!(
                            "Malformed number at line {}, column {}: {}",
                            self.line, self.column, err
                        );
                        errors.push(err);
                    }
                },
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.identifier()),
                '+' => self.push_token(&mut tokens, Token::Plus),
                '-' => self.push_token(&mut tokens, Token::Minus),
                '*' => self.push_token(&mut tokens, Token::Star),
                '/' => match self.peek() {
                    Some('/') => {
                        self.advance();
                        self.advance();

                        while let Some(c) = self.current_char {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    }
                    Some('*') => {
                        self.advance();
                        self.advance();

                        while let Some(c) = self.current_char {
                            if c == '*' && self.peek() == Some('/') {
                                self.advance();
                                self.advance();
                                break;
                            }
                            self.advance();
                        }
                    }
                    _ => {
                        self.push_token(&mut tokens, Token::Slash);
                    }
                },

                '%' => self.push_token(&mut tokens, Token::Percent),
                '^' => self.push_token(&mut tokens, Token::Caret),
                '(' => self.push_token(&mut tokens, Token::LParen),
                ')' => self.push_token(&mut tokens, Token::RParen),
                '[' => self.push_token(&mut tokens, Token::LBracket),
                ']' => self.push_token(&mut tokens, Token::RBracket),
                '{' => self.push_token(&mut tokens, Token::LBrace),
                '}' => self.push_token(&mut tokens, Token::RBrace),
                ',' => self.push_token(&mut tokens, Token::Comma),
                '!' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::ExclamationEqual);
                    } else {
                        self.advance();
                        tokens.push(Token::Exclamation);
                    }
                }
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::EqualEqual);
                    } else {
                        self.advance();
                        tokens.push(Token::Equal);
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::LessEqual);
                    } else {
                        self.advance();
                        tokens.push(Token::Less);
                    }
                }

                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::GreaterEqual);
                    } else {
                        self.advance();
                        tokens.push(Token::Greater);
                    }
                }
                ';' => self.push_token(&mut tokens, Token::Semicolon),
                c if c.is_whitespace() => self.whitespace(&mut tokens),
                _ => {
                    warn!(
                        "Unexpected character '{}' at line {}, column {}",
                        c, self.line, self.column
                    );
                    errors.push(LexerError::UnexpectedCharacter(c, self.line, self.column));
                    self.advance();
                }
            }
        }

        tokens.push(Token::Eof);

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(tokens)
        }
    }
}
