use super::super::super::ast::{
  Expression,
  Pattern,
};
use super::super::{
  Value,
  Scope,
  RuntimeError,
  transform,
  exec,
};

pub fn project(scope: &mut Scope, pattern: &Pattern, expression: &Expression) -> Result<Value, RuntimeError> {
  match transform(scope, pattern) {
    Ok(_) => exec(scope, expression),
    Err(e) => Err(e)
  }
}