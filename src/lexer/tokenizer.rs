// TODO: logging
use crate::lexer::{
    error::LexerError,
    token::{SpannedToken, Token},
};

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        log::debug!("Initializing lexer...");
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
        log::debug!("advance() called at line {}, column {}", self.line, self.column);
        if let Some(c) = self.current_char {
            if c == '\n' {
                log::debug!("advance() detected newline character");
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

    fn whitespace(&mut self, tokens: &mut Vec<SpannedToken>) {
        log::debug!("whitespace() called at line {}, column {}", self.line, self.column);

        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                if c == '\n' {
                    log::debug!("whitespace() detected newline character");
                    self.push_token(tokens, Token::Newline);
                } else if c == ' ' || c == '\t' {
                    log::debug!("whitespace() detected space or tab character");
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
        log::debug!("identifier() called at line {}, column {}", self.line, self.column);
        let mut id_str = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                log::debug!("identifier() adding character to identifier: {}", c);
                id_str.push(c);
                self.advance();
            } else {
                log::warn!("identifier() encountered non-identifier character: {}", c);
                break;
            }
        }

        Token::Identifier(id_str)
    }

    fn number(&mut self) -> Result<Token, LexerError> {
        log::debug!("number() called at line {}, column {}", self.line, self.column);
        let start_line = self.line;
        let start_col = self.column;

        let mut num_str = String::new();
        let mut has_dot = false;
        let mut has_exponent = false;

        while let Some(c) = self.current_char {
            match c {
                '0'..='9' => {
                    log::debug!("number() adding digit to number: {}", c);
                    num_str.push(c);
                    self.advance();
                }

                '.' => {
                    if has_dot || has_exponent {
                        log::warn!("number() detected invalid number format with multiple dots or dot after exponent");
                        num_str.push(c);
                        self.advance();

                        while let Some(nc) = self.current_char {
                            match nc {
                                '0'..='9' | '.' | 'e' | 'E' | '+' | '-' => {
                                    num_str.push(nc);
                                    self.advance();
                                }
                                _ => break,
                            }
                        }


                        return Err(LexerError::InvalidNumberFormat(
                            num_str, start_line, start_col,
                        ));
                    }

                    log::debug!("number() adding decimal point to number");
                    has_dot = true;
                    num_str.push(c);
                    self.advance();
                }

                'e' | 'E' => {

                    if has_exponent {
                        num_str.push(c);
                        self.advance();
                        log::warn!("number() detected invalid number format with multiple exponents");
                        return Err(LexerError::InvalidNumberFormat(
                            num_str, start_line, start_col,
                        ));
                    }


                    log::debug!("number() adding exponent to number");
                    has_exponent = true;
                    num_str.push(c);
                    self.advance();

                    if let Some(sign @ ('+' | '-')) = self.current_char {
                        log::debug!("number() adding exponent sign to number: {}", sign);
                        num_str.push(sign);
                        self.advance();
                    }
                }

                _ => break,
            }
        }

        num_str
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| LexerError::InvalidNumberFormat(num_str, start_line, start_col))
    }

    fn push_token(&mut self, tokens: &mut Vec<SpannedToken>, token: Token) {
        let spanned = token.span(self.line, self.column, self.pos);
        tokens.push(spanned);
        self.advance();
    }

    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken>, Vec<LexerError>> {
        log::debug!("tokenize() called");
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        while let Some(c) = self.current_char {
            let start_line = self.line;
            let start_col = self.column;
            let start_pos = self.pos;

            match c {
                '0'..='9' | '.' => match self.number() {
                    Ok(token) => {
                        tokens.push(token.span(start_line, start_col, start_pos));
                    }
                    Err(err) => {
                        errors.push(err);
                    }
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let token = self.identifier();
                    tokens.push(token.span(start_line, start_col, start_pos));
                }
                '+' => self.push_token(&mut tokens, Token::Plus),
                '-' => self.push_token(&mut tokens, Token::Minus),
                '*' => self.push_token(&mut tokens, Token::Star),
                '/' => {
                    if self.peek() == Some('/') {
                        self.advance();
                        self.advance();
                        while let Some(c) = self.current_char {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else if self.peek() == Some('*') {
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
                    } else {
                        self.push_token(&mut tokens, Token::Slash);
                    }
                }
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
                        tokens.push(Token::ExclamationEqual.span(start_line, start_col, start_pos));
                        self.advance();
                    } else {
                        self.push_token(&mut tokens, Token::Exclamation);
                    }
                }
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::EqualEqual.span(start_line, start_col, start_pos));
                        self.advance();
                    } else {
                        self.push_token(&mut tokens, Token::Equal);
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::LessEqual.span(start_line, start_col, start_pos));
                        self.advance();
                    } else {
                        self.push_token(&mut tokens, Token::Less);
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token::GreaterEqual.span(start_line, start_col, start_pos));
                        self.advance();
                    } else {
                        self.push_token(&mut tokens, Token::Greater);
                    }
                }
                ';' => self.push_token(&mut tokens, Token::Semicolon),
                c if c.is_whitespace() => self.whitespace(&mut tokens),
                _ => {
                    errors.push(LexerError::UnexpectedCharacter(c, self.line, self.column));
                    self.advance();
                }
            }
        }

        tokens.push(Token::Eof.span(self.line, self.column, self.pos));

        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(tokens)
        }
    }
}
