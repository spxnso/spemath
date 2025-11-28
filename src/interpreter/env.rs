use std::collections::HashMap;

use crate::interpreter::value::Value;

#[derive(Clone)]
pub struct Env {
    pub variables: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}