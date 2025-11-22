use crate::lexer::token::Token;
use core::panic;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            pos: 0,
            current_char: None,
        };
        lexer.current_char = lexer.input.chars().nth(0);
        lexer
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.input.chars().nth(self.pos)
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos + 1)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
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

    fn number(&mut self) -> Token {
        let mut num_str = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) || c == '.' {
                num_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token::Number(num_str.parse::<f64>().unwrap())
    }

    fn push_token(&mut self, tokens: &mut Vec<Token>, token: Token) {
        tokens.push(token);
        self.advance();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char {
            match c {
                '0'..'9' | '.' => tokens.push(self.number()),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.identifier()),
                '+' => self.push_token(&mut tokens, Token::Plus),
                '-' => self.push_token(&mut tokens, Token::Minus),
                '*' => self.push_token(&mut tokens, Token::Star),
                '/' => {
                    match self.peek() {
                        Some('/') => {
                            while let Some(ch) = self.current_char {
                                if ch == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                        }
                        Some('*') => {
                            self.advance();
                            self.advance();
                            while let Some(ch) = self.current_char {
                                if ch == '*' && self.peek() == Some('/') {
                                    self.advance();
                                    self.advance();
                                    break;
                                }
                                self.advance();
                            }

                        }
                        _ => self.push_token(&mut tokens, Token::Slash),
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
                c if c.is_whitespace() => self.skip_whitespace(),
                _ => panic!("Unknown character: {:?}", c),
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}
