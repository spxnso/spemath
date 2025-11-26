use thiserror::Error;

use crate::{lexer::token::Token, parser::ast::Expr};

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ParserError {

    #[error("Unexpected token {0:?} at position {1}")]
    UnexpectedToken(Token, usize),

    #[error("Expected token {0:?} but found {1:?} at position {2}")]
    ExpectedToken(Token, Token, usize),

    #[error("Invalid assignment, left-hand side must be a variable at position {0}")]
    InvalidAssignment(usize, Expr, Token),

    #[error("Invalid function parameters at position {0}")]
    InvalidFunctionParameter(usize, Expr),

    #[error("Invalid function definition at position {0}")]
    InvalidFunctionDefinition(usize, Token),

    #[error("Unexpected end of input at position {0}")]
    UnexpectedEOF(usize, Token),
}