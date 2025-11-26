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

    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    println!("{:#?}", ast);
}
