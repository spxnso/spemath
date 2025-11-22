// TODO: Refactor double operator

use crate::lexer::token::Token;
use core::panic;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let current_char = chars.get(0).copied();
        Self {
            chars,
            pos: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.chars.get(self.pos).copied();
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
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

        if let Some('e') | Some('E') = self.current_char {
            num_str.push(self.current_char.unwrap());
            self.advance();

            if let Some('+') | Some('-') = self.current_char {
                num_str.push(self.current_char.unwrap());
                self.advance();
            }

            let mut exponent_digits = 0;
            while let Some(c) = self.current_char {
                if c.is_digit(10) {
                    num_str.push(c);
                    self.advance();
                    exponent_digits += 1;
                } else {
                    break;
                }
            }

            if exponent_digits == 0 {
                panic!("Invalid scientific notation: missing digits in exponent");
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
                '0'..='9' | '.' => tokens.push(self.number()),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.identifier()),
                '+' => self.push_token(&mut tokens, Token::Plus),
                '-' => self.push_token(&mut tokens, Token::Minus),
                '*' => self.push_token(&mut tokens, Token::Star),
                '/' => match self.peek() {
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
