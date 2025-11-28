use std::fs;

mod interpreter;
mod lexer;
mod parser;


use crate::interpreter::eval::Evaluator;
use crate::interpreter::value::Value;
use crate::lexer::tokenizer::Lexer;
use crate::parser::pratt::Parser;


fn main() {
    env_logger::init();

    let source = fs::read_to_string("input.spemath")
        .expect("Could not read input.spemath");

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(errors) => {
            for err in errors {
                eprintln!("{:?}", err);
            }
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let exprs = match parser.parse() {
        Ok(e) => e,
        Err(errors) => {
            for err in errors {
                eprintln!("Error: {}", err);
            }
            return;
        }
    };

    let mut evaluator = Evaluator::new();
    for expr in exprs {
        match evaluator.eval(&expr) {
            Ok(Value::Unit) => {}
            Ok(value) => println!("{:?}", value),
            Err(err) => eprintln!("Evaluation error: {}", err),
        }
    }
}
