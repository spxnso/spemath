use crate::lexer::tokenizer::Lexer;
use crate::parser::pratt::Parser;
use crate::interpreter::eval::Evaluator;
use crate::interpreter::value::Value;

pub fn run_source(source: &str) -> Result<String, String> {
    // 1. LEXER
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|errs| {
        errs.into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    // 2. PARSER
    let mut parser = Parser::new(tokens);
    let exprs = parser.parse().map_err(|errs| {
        errs.into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    // 3. EVALUATOR
    let mut evaluator = Evaluator::new();
    let mut output = String::new();

    for expr in exprs {
        match evaluator.eval(&expr) {
            Ok(Value::Unit) => {}
            Ok(value) => output.push_str(&format!("{:?}\n", value)),
            Err(err) => output.push_str(&format!("Runtime Error: {}\n", err)),
        }
    }

    Ok(output)
}
