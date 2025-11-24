use crate::{lexer::token::Token, parser::ast::Expr};

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum Precedence {
    Lowest = 0,
    Assignment = 1, // =
    Comparison = 2, // == != < > <= >=
    Sum = 3,        // + -
    Product = 4,    // * / %
    Power = 5,      // ^
    Prefix = 6,     // -x
    Call = 7,       // f(x)
}

impl Precedence {
    fn from_u8(value: u8) -> Precedence {
        match value {
            1 => Precedence::Assignment,
            2 => Precedence::Comparison,
            3 => Precedence::Sum,
            4 => Precedence::Product,
            5 => Precedence::Power,
            6 => Precedence::Prefix,
            7 => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

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
        if self.pos == 0 {
            None
        } else {
            self.tokens.get(self.pos - 1)
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    // fn peek(&self) -> Option<&Token> {
    //     self.tokens.get(self.pos + 1)
    // }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: &Token) {
        if self.current() == Some(expected) {
            self.advance();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.current());
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        self.program()
    }

    fn program(&mut self) -> Vec<Expr> {
        let mut nodes = Vec::new();

        while let Some(tok) = self.current().cloned() {
            match tok {
                Token::Eof => break,
                Token::Newline | Token::Semicolon => {
                    self.advance();
                }
                _ => {
                    if let Some(expr) = self.expression(Precedence::Lowest) {
                        nodes.push(expr);
                    } else {
                        panic!("Unexpected token: {:?}", tok);
                    }
                }
            }
        }

        nodes
    }

    fn expression(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left = self.prefix()?;

        while let Some(token) = self.current().cloned() {
            match token {
                Token::Eof | Token::Newline => break,

                Token::Equal => {
                    if let Expr::Identifier(name) = left {
                        self.advance();
                        let right = self.expression(Precedence::Assignment)?;
                        left = Expr::Assignment {
                            target: name,
                            value: Box::new(right),
                        };
                        continue;
                    } else {
                        panic!("Left side of assignment must be an identifier");
                    }
                }

                t if self.is_implicit_multiplication(&t) => {
                    let token_prec = Precedence::from_token(&Token::Star);
                    if (token_prec as u8) <= (precedence as u8) {
                        break;
                    }
                    let right = self.expression(token_prec)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: Token::Star,
                        right: Box::new(right),
                    };
                    continue;
                }

                Token::LParen => {
                    left = self.call(left)?;
                    continue;
                }

                _ => {
                    let token_prec = Precedence::from_token(&token);

                    let should_break = match token {
                        Token::Caret => (token_prec as u8) < (precedence as u8),
                        _ => (token_prec as u8) <= (precedence as u8),
                    };

                    if should_break {
                        break;
                    }

                    self.advance();

                    let next_prec = match token {
                        Token::Caret => token_prec,
                        _ => Precedence::from_u8((token_prec as u8) + 1),
                    };

                    let right = self.expression(next_prec)?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: token,
                        right: Box::new(right),
                    };
                }
            }
        }

        Some(left)
    }

    fn prefix(&mut self) -> Option<Expr> {
        match self.current().cloned()? {
            Token::Number(n) => {
                self.advance();
                Some(Expr::Number(n))
            }

            Token::Identifier(name) => {
                self.advance();

                if self.current() == Some(&Token::LParen) {
                    let params = self.params();

                    if self.current() == Some(&Token::Equal) {
                        self.advance();
                        let body = self.expression(Precedence::Lowest)?;
                        return Some(Expr::FunctionDef {
                            name,
                            args: params,
                            body: Box::new(body),
                        });
                    }

                    let args = params.into_iter().map(Expr::Identifier).collect();

                    return Some(Expr::Call {
                        function: Box::new(Expr::Identifier(name)),
                        args,
                    });
                }

                Some(Expr::Identifier(name))
            }

            Token::Minus | Token::Plus => {
                let op = self.current().cloned().unwrap();
                self.advance();
                Some(Expr::Unary {
                    op,
                    expr: Box::new(self.expression(Precedence::Prefix)?),
                })
            }

            Token::LParen => {
                self.advance();
                let expr = self.expression(Precedence::Lowest)?;
                self.expect(&Token::RParen);
                Some(expr)
            }

            _ => None,
        }
    }

    fn call(&mut self, function: Expr) -> Option<Expr> {
        self.expect(&Token::LParen);

        let mut args = Vec::new();

        if self.current() != Some(&Token::RParen) {
            loop {
                let arg = self.expression(Precedence::Lowest)?;
                args.push(arg);

                if self.current() == Some(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(&Token::RParen);

        Some(Expr::Call {
            function: Box::new(function),
            args,
        })
    }

    fn params(&mut self) -> Vec<String> {
        let mut params = Vec::new();

        self.expect(&Token::LParen);

        if self.current() != Some(&Token::RParen) {
            loop {
                match self.current().cloned() {
                    Some(Token::Identifier(p)) => {
                        params.push(p);
                        self.advance();
                    }
                    Some(Token::Number(n)) => {
                        params.push(n.to_string());
                        self.advance();
                    }
                    _ => panic!("Expected parameter name"),
                }

                if self.current() == Some(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.expect(&Token::RParen);
        params
    }

    fn is_implicit_multiplication(&self, token: &Token) -> bool {
        use Token::*;

        match token {
            Identifier(_) | LParen => {
                matches!(
                    self.previous(),
                    Some(Number(_)) | Some(Identifier(_)) | Some(RParen)
                )
            }
            Number(_) => matches!(self.previous(), Some(RParen)),

            _ => false,
        }
    }
}
