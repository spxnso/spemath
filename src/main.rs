use std::fs;

use crate::lexer::tokenizer::Lexer;
use crate::parser::pratt::Parser;

mod lexer;
mod parser;
fn main() {
    let source = fs::read_to_string("input.spemath").expect("Could not read input.spemath");
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    tokens.iter().for_each(|token| println!("{:?}", token));

    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    println!("{:#?}", ast);
}
