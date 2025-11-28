use wasm_bindgen::prelude::*;

pub mod core;
pub mod lexer;
pub mod parser;
pub mod interpreter;

use crate::core::runtime::run_source;

#[wasm_bindgen]
pub fn run_code(source: &str) -> String {
    match run_source(source) {
        Ok(output) => output,
        Err(err) => format!("Error: {}", err),
    }
}
