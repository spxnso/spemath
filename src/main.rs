use std::fs;

mod lexer;
fn main() {
    let source = fs::read_to_string("input.spemath").expect("Could not read input.spemath");
    let mut lexer = lexer::lexer::Lexer::new(&source);
    lexer.tokenize().iter().for_each(|token| println!("{:?}", token));
}
