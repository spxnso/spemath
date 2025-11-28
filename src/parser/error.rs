use thiserror::Error;

use crate::{lexer::token::Token, parser::ast::Expr};

#[derive(Error, Debug, Clone)]
pub enum ParserError {
    #[error("line {line}, col {col}: Unexpected token '{}'", found.description())]
    UnexpectedToken {
        found: Token,
        line: usize,
        col: usize,
        pos: usize,
    },

    #[error("line {line}, col {col}: Expected {}, found {}", expected.description(), found.description())]
    ExpectedToken {
        expected: Token,
        found: Token,
        line: usize,
        col: usize,
        pos: usize,
    },

    #[error("line {line}, col {col}: Unexpected end of input, expected '{expected}'")]
    UnexpectedEof {
        expected: String,
        line: usize,
        col: usize,
        pos: usize,
    },

    #[error(
        "line {line}, col {col}: Cannot assign to '{target:?}', left-hand side must be a variable"
    )]
    InvalidAssignment {
        target: Expr,
        line: usize,
        col: usize,
        pos: usize,
    },

    #[error("line {line}, col {col}: Function parameter must be an identifier, found '{param:?}'")]
    InvalidFunctionParameter {
        param: Expr,
        line: usize,
        col: usize,
        pos: usize,
    },

    #[error("line {line}, col {col}: Invalid function definition syntax")]
    InvalidFunctionDefinition { line: usize, col: usize, pos: usize },
}
