use crate::lexer::token::SpannedToken;
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
    tokens: Vec<SpannedToken>,
    pos: usize,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        log::debug!("Initializing parser...");
        Parser {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    fn position(&self) -> (usize, usize, usize) {
        if let Some(spanned) = self.tokens.get(self.pos) {
            (spanned.span.line, spanned.span.col, spanned.span.pos)
        } else if let Some(last) = self.tokens.last() {
            (last.span.line, last.span.col, last.span.pos)
        } else {
            (1, 1, 0)
        }
    }

    fn error(&mut self, error: ParserError) {
        self.errors.push(error);
    }

    fn previous(&self) -> Option<&Token> {
        let mut i = self.pos;
        while i > 0 {
            i -= 1;
            if let Some(spanned) = self.tokens.get(i) {
                if !matches!(spanned.value, Token::Whitespace) {
                    return Some(&spanned.value);
                }
            }
        }
        None
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|spanned| &spanned.value)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.current() {
            match token {
                Token::Semicolon | Token::Newline | Token::Eof => {
                    self.advance();
                    return;
                }
                _ => self.advance(),
            }
        }
    }

    fn has_whitespace_before(&self) -> bool {
        if self.pos > 0 {
            matches!(
                self.tokens.get(self.pos - 1).map(|s| &s.value),
                Some(Token::Whitespace)
            )
        } else {
            false
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParserError> {
        log::debug!("expect({:?}) at pos {}", expected, self.pos);
        self.whitespace();

        let current = self.current().cloned().unwrap_or(Token::Eof);

        if &current == expected {
            self.advance();
            log::debug!("expect({:?}) succeeded", expected);
            Ok(())
        } else {
            log::warn!("expect({:?}) failed, found {:?}", expected, current);

            let (line, col, pos) = self.position();

            if current == Token::Eof {
                Err(ParserError::UnexpectedEof {
                    expected: expected.clone().description(),
                    line,
                    col,
                    pos,
                })
            } else {
                Err(ParserError::ExpectedToken {
                    expected: expected.clone(),
                    found: current,
                    line,
                    col,
                    pos,
                })
            }
        }
    }

    fn whitespace(&mut self) {
        while let Some(Token::Whitespace) = self.current() {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, Vec<ParserError>> {
        let exprs = self.program();

        if self.errors.is_empty() {
            Ok(exprs)
        } else {
            Err(self.errors.clone())
        }
    }

    fn program(&mut self) -> Vec<Expr> {
        let mut nodes = Vec::new();

        while let Some(tok) = self.current().cloned() {
            match tok {
                Token::Eof => break,
                Token::Newline | Token::Semicolon | Token::Whitespace => {
                    self.advance();
                }
                _ => match self.expression(Precedence::Lowest) {
                    Ok(expr) => nodes.push(expr),
                    Err(err) => {
                        self.error(err);
                        self.synchronize();
                    }
                },
            }
        }

        nodes
    }

    fn expression(&mut self, precedence: Precedence) -> Result<Expr, ParserError> {
        let mut left = self.prefix()?;

        log::debug!(
            "expression() at pos {}, precedence {:?}",
            self.pos,
            precedence
        );

        loop {
            self.whitespace();

            let Some(token) = self.current().cloned() else {
                break;
            };

            match token {
                Token::Eof | Token::Newline | Token::Semicolon => {
                    log::debug!("expression() reached end of expression at pos {}", self.pos);
                    break;
                }

                Token::Equal => {
                    log::debug!("expression() found assignment operator at pos {}", self.pos);
                    let token_prec = Precedence::Assignment;
                    if token_prec <= precedence {
                        break;
                    }

                    match left {
                        Expr::Call { function, args } => {
                            log::debug!("expression() found function call at pos {}", self.pos);
                            if let Expr::Identifier(name) = *function {
                                log::debug!(
                                    "expression() parsing function definition for '{}' at pos {}",
                                    name,
                                    self.pos
                                );
                                let mut params = Vec::new();

                                for arg in args {
                                    if let Expr::Identifier(param_name) = arg {
                                        params.push(param_name);
                                    } else {
                                        log::warn!(
                                            "expression() invalid function parameter at pos {}",
                                            self.pos
                                        );
                                        let (line, col, pos) = self.position();
                                        return Err(ParserError::InvalidFunctionParameter {
                                            param: arg,
                                            line,
                                            col,
                                            pos,
                                        });
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
                                log::warn!(
                                    "expression() invalid function definition target at pos {}",
                                    self.pos
                                );
                                let (line, col, pos) = self.position();
                                return Err(ParserError::InvalidFunctionDefinition {
                                    line,
                                    col,
                                    pos,
                                });
                            }
                        }
                        Expr::Identifier(name) => {
                            log::debug!(
                                "expression() parsing assignment to '{}' at pos {}",
                                name,
                                self.pos
                            );
                            self.advance();
                            let value = self.expression(Precedence::Assignment)?;
                            left = Expr::Assignment {
                                target: name,
                                value: Box::new(value),
                            };
                        }
                        _ => {
                            log::warn!(
                                "expression() invalid assignment target at pos {}",
                                self.pos
                            );
                            let (line, col, pos) = self.position();

                            return Err(ParserError::InvalidAssignment {
                                target: left,
                                line,
                                col,
                                pos,
                            });
                        }
                    }
                }

                Token::LParen => {
                    log::debug!("expression() found '(' at pos {}", self.pos);
                    if matches!(left, Expr::Identifier(_))
                        && !self.has_whitespace_before()
                        && precedence < Precedence::Product
                    {
                        log::debug!("expression() found function call at pos {}", self.pos);
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
                    log::debug!(
                        "expression() found implicit multiplication at pos {}",
                        self.pos
                    );
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
                    log::debug!(
                        "expression() found infix operator {:?} at pos {}",
                        token,
                        self.pos
                    );
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

        log::debug!("expression() returning {:?}", left);
        Ok(left)
    }

    fn prefix(&mut self) -> Result<Expr, ParserError> {
        log::debug!("prefix() at pos {}", self.pos);
        self.whitespace();

        match self.current().cloned() {
            Some(Token::Number(n)) => {
                log::debug!("prefix() found number {:?}", n);
                self.advance();
                Ok(Expr::Number(n))
            }

            Some(Token::Identifier(name)) => {
                log::debug!("prefix() found identifier {:?}", name);
                self.advance();
                Ok(Expr::Identifier(name))
            }

            Some(Token::Minus) | Some(Token::Plus) => {
                log::debug!("prefix() found unary operator {:?}", self.current());
                let op = self.current().cloned().unwrap();
                self.advance();
                let expr = self.expression(Precedence::Prefix)?;
                Ok(Expr::Unary {
                    op,
                    expr: Box::new(expr),
                })
            }

            Some(Token::LParen) => {
                log::debug!("prefix() found grouped expression");
                self.advance();
                let expr = self.expression(Precedence::Lowest)?;
                self.expect(&Token::RParen)?;
                log::debug!("prefix() done grouping expression: {:?}", expr);
                Ok(expr)
            }

            Some(token) => {
                log::warn!("prefix() found unexpected token {:?}", token);
                let (line, col, pos) = self.position();
                Err(ParserError::UnexpectedToken {
                    found: token,
                    line,
                    col,
                    pos,
                })
            }

            None => {
                log::warn!("prefix() found unexpected end of input");
                let (line, col, pos) = self.position();
                Err(ParserError::UnexpectedEof {
                    expected: "expression".into(),
                    line,
                    col,
                    pos,
                })
            }
        }
    }

    fn call(&mut self, function: Expr) -> Result<Expr, ParserError> {
        log::debug!("call() at pos {}", self.pos);
        let args = self.arguments()?;
        Ok(Expr::Call {
            function: Box::new(function),
            args,
        })
    }

    fn arguments(&mut self) -> Result<Vec<Expr>, ParserError> {
        log::debug!("arguments() at pos {}", self.pos);
        self.expect(&Token::LParen)?;
        let mut args = Vec::new();

        self.whitespace();
        if self.current() != Some(&Token::RParen) {
            loop {
                args.push(self.expression(Precedence::Lowest)?);

                self.whitespace();
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
        log::debug!("is_implicit_multiplication() at pos {}", self.pos);
        if self.has_whitespace_before() {
            return false;
        }

        match token {
            Token::Identifier(_) => {
                matches!(
                    self.previous(),
                    Some(Token::Number(_)) | Some(Token::RParen)
                )
            }

            Token::Number(_) => {
                matches!(
                    self.previous(),
                    Some(Token::RParen) | Some(Token::Identifier(_))
                )
            }

            _ => false,
        }
    }
}
