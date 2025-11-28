use crate::parser::ast::Expr;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Function(FunctionValue),
    Unit,
}

#[derive(Clone, Debug)]
pub struct FunctionValue {
    pub params: Vec<String>,
    pub body: Expr,
}