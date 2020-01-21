use super::super::{
  Scope,
  Value,
  Value::{
    None
  },
  exec,
  RuntimeError,
  RuntimeError::{
    InvalidValueError
  }
};
use super::super::super::ast::Expression;

pub fn block(scope: &mut Scope, expressions: &Vec<Box<Expression>>) -> Result<Value, RuntimeError> {
  // todo: should handle the return case..
  let mut last = None;
  for expression in expressions.iter() {
    match exec(scope, &expression) {
      Err(e) => return Err(e),
      Ok(v) => { last = v; }
    }
  }

  Ok(last)
}
