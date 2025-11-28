use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum EvalError {
    #[error("Unknown variable: '{0}'")]
    UnknownVariable(String),

    #[error("Unsupported expression: {0}")]
    UnsupportedExpression(String),

    #[error("Invalid unary operation: '{0:?}'")]
    InvalidUnary(crate::lexer::token::Token),
}
