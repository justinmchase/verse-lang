use super::super::super::ast::{
  Pattern
};
use super::super::{
  Value,
  Scope,
  transform,
  RuntimeError
};

pub fn var(scope: &mut Scope, name: String, pattern: &Pattern) -> Result<Value, RuntimeError> {
  match transform(scope, pattern) {
    Ok(v) => {
      scope.vars.insert(name, v.clone());
      Ok(v)
    },
    Err(e) => Err(e)
  }
}