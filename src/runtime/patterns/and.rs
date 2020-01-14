use super::super::super::ast::{
  Pattern
};
use super::super::{
  Value,
  Scope,
  transform,
  RuntimeError
};

pub fn and(scope: &mut Scope, patterns: &Vec<Box<Pattern>>) -> Result<Value, RuntimeError> {
  let i = scope.pos();
  let mut results = vec![];
  for p in patterns.iter() {
    match transform(scope, p) {
      Ok(v) => {
        results.push(v);
      },
      Err(e) => {
        scope.mov(i);
        return Err(e);
      }
    }
  }
  Ok(Value::Array(results))
}
