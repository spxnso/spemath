// An attempt of a simple evaluator
use crate::{
    interpreter::{
        env::Env,
        error::EvalError,
        value::{FunctionValue, Value},
    },
    lexer::token::Token,
    parser::ast::Expr,
};

pub struct Evaluator {
    pub env: Env,
}

impl Evaluator {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn eval(&mut self, expr: &Expr) -> Result<Value, EvalError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::Identifier(name) => self
                .env
                .get(name)
                .cloned()
                .ok_or(EvalError::UnknownVariable(name.clone())),

            Expr::Unary { op, expr } => {
                let v = self.eval(expr)?;
                match (op, v) {
                    (Token::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (Token::Plus, Value::Number(n)) => Ok(Value::Number(n)),
                    _ => Err(EvalError::InvalidUnary(op.clone())),
                }
            }

            Expr::Binary { left, op, right } => {
                // TODO: Equation solving
                let l = self.eval(left)?;
                let r = self.eval(right)?;

                match (op, l, r) {
                    (Token::Plus, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Token::Minus, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                    (Token::Star, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                    (Token::Slash, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
                    (Token::Caret, Value::Number(a), Value::Number(b)) => {
                        Ok(Value::Number(a.powf(b)))
                    }
                    _ => Err(EvalError::UnsupportedExpression(format!(
                        "Unsupported binary operation: {:?} {:?} {:?}",
                        left, op, right
                    ))),
                }
            }

            Expr::Assignment { target, value } => {
                let evaluated = self.eval(value)?;
                self.env.set(target.clone(), evaluated.clone());
                Ok(Value::Unit)
            }

            Expr::Function { name, args, body } => {
                let f = Value::Function(FunctionValue {
                    params: args.clone(),
                    body: *body.clone(),
                });

                self.env.set(name.clone(), f.clone());
                Ok(Value::Unit)
            }

            Expr::Call { function, args } => {
                let func_value = self.eval(function)?;
                // TODO: Handle built-in functions
                match func_value {
                    Value::Function(func) => {
                        if func.params.len() != args.len() {
                            return Err(EvalError::UnsupportedExpression(format!(
                                "Function expected {} arguments but got {}",
                                func.params.len(),
                                args.len()
                            )));
                        }

                        let mut new_env = self.env.clone();
                        for (param, arg_expr) in func.params.iter().zip(args.iter()) {
                            let arg_value = self.eval(arg_expr)?;
                            new_env.set(param.clone(), arg_value);
                        }

                        let mut evaluator = Evaluator { env: new_env };
                        evaluator.eval(&func.body)
                    }
                    _ => Err(EvalError::UnsupportedExpression(format!(
                        "Attempted to call a non-function value: {:?}",
                        func_value
                    ))),
                }
            }
        }
    }
}
