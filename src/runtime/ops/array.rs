use super::super::{
  Scope,
  Value,
  RuntimeError,
  exec
};
use super::super::super::ast::{
  Expression,
  Pattern
};

pub fn array(scope: Scope, exp: &Vec<Box<Expression>>) -> Result<Value, RuntimeError> {
  let mut values = vec![];
  for e in exp.iter() {
    match exec(scope.clone(), e) {
      Ok(v) => {
        values.push(v);
      },
      Err(e) => {
        return Err(e)
      }
    }
  }

  Ok(Value::Array(values))
}
