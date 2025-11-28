use std::fs;

use crate::lexer::tokenizer::Lexer;
use crate::parser::pratt::Parser;

mod lexer;
mod parser;
fn main() {
    env_logger::init();
    let source = fs::read_to_string("input.spemath").expect("Could not read input.spemath");

    log::info!("Starting lexer...");
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => {
            log::info!("Lexer produced {} token(s)", tokens.len());
            tokens
        }
        Err(errors) => {
            log::info!("Lexer encountered {} error(s)", errors.len());
            for err in errors {
                log::error!("{:?}", err);
            }
            return;
        }
    };

    log::debug!("Tokens: {:#?}", tokens);

    log::info!("Starting parser...");
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(exprs) => {
           log::info!("Parser produced {} expression(s)", exprs.len());
           log::debug!("AST: {:#?}", exprs);
        }
        Err(errors) => {
            for error in errors {
                log::error!("Error: {}", error);
            }
        }
    }

}
