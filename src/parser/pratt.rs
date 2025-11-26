use crate::parser::error::ParserError;
use crate::{lexer::token::Token, parser::ast::Expr};

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum Precedence {
    Lowest = 0,
    Assignment = 1,
    Comparison = 2,
    Sum = 3,
    Product = 4,
    Power = 5,
    Prefix = 6,
    Call = 7,
}

impl Precedence {
    fn from_token(token: &Token) -> Precedence {
        match token {
            Token::Equal => Precedence::Assignment,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Star | Token::Slash | Token::Percent => Precedence::Product,
            Token::Caret => Precedence::Power,
            Token::EqualEqual
            | Token::ExclamationEqual
            | Token::Less
            | Token::Greater
            | Token::LessEqual
            | Token::GreaterEqual => Precedence::Comparison,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn previous(&self) -> Option<&Token> {
        let mut i = self.pos;
        while i > 0 {
            i -= 1;
            if let Some(token) = self.tokens.get(i) {
                if !matches!(token, Token::Whitespace) {
                    return Some(token);
                }
            }
        }
        None
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(Token::Whitespace) = self.current() {
            self.advance();
        }
    }

    fn has_whitespace_before(&self) -> bool {
        if self.pos > 0 {
            matches!(self.tokens.get(self.pos - 1), Some(Token::Whitespace))
        } else {
            false
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParserError> {
        self.skip_whitespace();
        if self.current() == Some(expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(
                self.current().cloned().unwrap_or(Token::Eof),
                self.pos,
            ))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, ParserError> {
        self.program()
    }

    fn program(&mut self) -> Result<Vec<Expr>, ParserError> {
        let mut nodes = Vec::new();

        while let Some(tok) = self.current().cloned() {
            match tok {
                Token::Eof => break,
                Token::Newline | Token::Semicolon | Token::Whitespace => {
                    self.advance();
                }
                _ => {
                    let expr = self.expression(Precedence::Lowest)?;
                    nodes.push(expr);
                }
            }
        }

        Ok(nodes)
    }

    fn expression(&mut self, precedence: Precedence) -> Result<Expr, ParserError> {
        let mut left = self.prefix()?;

        loop {
            self.skip_whitespace();

            let Some(token) = self.current().cloned() else {
                break;
            };

            match token {
                Token::Eof | Token::Newline | Token::Semicolon => break,

                Token::Equal => {
                    let token_prec = Precedence::Assignment;
                    if token_prec <= precedence {
                        break;
                    }

                    match left {
                        Expr::Call { function, args } => {
                            if let Expr::Identifier(name) = *function {
                                let mut params = Vec::new();

                                for arg in args {
                                    if let Expr::Identifier(param_name) = arg {
                                        params.push(param_name);
                                    } else {
                                        return Err(ParserError::InvalidFunctionParameter(
                                            self.pos, arg,
                                        ));
                                    }
                                }

                                self.advance();
                                let body = self.expression(Precedence::Assignment)?;
                                left = Expr::Function {
                                    name,
                                    args: params,
                                    body: Box::new(body),
                                };
                            } else {
                                return Err(ParserError::InvalidFunctionDefinition(
                                    self.pos, token,
                                ));
                            }
                        }
                        Expr::Identifier(name) => {
                            self.advance();
                            let value = self.expression(Precedence::Assignment)?;
                            left = Expr::Assignment {
                                target: name,
                                value: Box::new(value),
                            };
                        }
                        _ => {
                            return Err(ParserError::InvalidAssignment(self.pos, left, token));
                        }
                    }
                }

                Token::LParen => {
                    if matches!(left, Expr::Identifier(_)) 
                        && !self.has_whitespace_before()
                        && precedence < Precedence::Product
                    {
                        left = self.call(left)?;
                    } else if !self.has_whitespace_before() {
                        let token_prec = Precedence::Product;
                        if token_prec <= precedence {
                            break;
                        }

                        let right = self.expression(token_prec)?;
                        left = Expr::Binary {
                            left: Box::new(left),
                            op: Token::Star,
                            right: Box::new(right),
                        };
                    } else {
                        break;
                    }
                }

                t if self.is_implicit_multiplication(&t) => {
                    let token_prec = Precedence::Product;
                    if token_prec < precedence {
                        break;
                    }

                    let right = self.expression(token_prec)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: Token::Star,
                        right: Box::new(right),
                    };
                }

                _ => {
                    let token_prec = Precedence::from_token(&token);

                    let should_break = match token {
                        Token::Caret => token_prec < precedence,
                        _ => token_prec <= precedence,
                    };

                    if should_break {
                        break;
                    }

                    self.advance();

                    let right = self.expression(token_prec)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: token,
                        right: Box::new(right),
                    };
                }
            }
        }

        Ok(left)
    }

    fn prefix(&mut self) -> Result<Expr, ParserError> {
        self.skip_whitespace();

        match self.current().cloned() {
            Some(Token::Number(n)) => {
                self.advance();
                Ok(Expr::Number(n))
            }

            Some(Token::Identifier(name)) => {
                self.advance();
                Ok(Expr::Identifier(name))
            }

            Some(Token::Minus) | Some(Token::Plus) => {
                let op = self.current().cloned().unwrap();
                self.advance();
                let expr = self.expression(Precedence::Prefix)?;
                Ok(Expr::Unary {
                    op,
                    expr: Box::new(expr),
                })
            }

            Some(Token::LParen) => {
                self.advance();
                let expr = self.expression(Precedence::Lowest)?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }

            _ => Err(ParserError::UnexpectedToken(
                self.current().cloned().unwrap_or(Token::Eof),
                self.pos,
            )),
        }
    }

    fn call(&mut self, function: Expr) -> Result<Expr, ParserError> {
        let args = self.arguments()?;
        Ok(Expr::Call {
            function: Box::new(function),
            args,
        })
    }

    fn arguments(&mut self) -> Result<Vec<Expr>, ParserError> {
        self.expect(&Token::LParen)?;
        let mut args = Vec::new();

        self.skip_whitespace();
        if self.current() != Some(&Token::RParen) {
            loop {
                args.push(self.expression(Precedence::Lowest)?);

                self.skip_whitespace();
                if self.current() == Some(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(&Token::RParen)?;
        Ok(args)
    }

    fn is_implicit_multiplication(&self, token: &Token) -> bool {
        use Token::*;

        if self.has_whitespace_before() {
            return false;
        }

        match token {
            LParen => {
                matches!(
                    self.previous(),
                    Some(Number(_)) | Some(Identifier(_)) | Some(RParen)
                )
            }

            Identifier(_) => {
                matches!(self.previous(), Some(Number(_)) | Some(RParen))
            }

            Number(_) => matches!(self.previous(), Some(RParen) | Some(Identifier(_))),
            _ => false,
        }
    }
}
