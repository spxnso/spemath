use crate::lexer::token::Token;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Identifier(String),
    Assignment {
        target: String,
        value: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>
    },
    Unary {
        op: Token,
        expr: Box<Expr>
    },
    Call {
        function: Box<Expr>,
        args: Vec<Expr>
    },
    Function {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
    }
}
