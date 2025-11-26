use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedCharacter(char, usize, usize),

    #[error("Invalid number format '{0}' at line {1}, column {2}")]
    InvalidNumberFormat(String, usize, usize),
}