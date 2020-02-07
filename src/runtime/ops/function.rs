use super::super::{
  Scope,
  Value,
  RuntimeError,
};
use super::super::super::ast::{
  Expression,
  Pattern
};

pub fn function(scope: Scope, p: &Pattern, e: &Expression) -> Result<Value, RuntimeError> {
  let vars = scope.clone_vars();
  Ok(Value::Function(Box::new(p.clone()), Box::new(e.clone()), vars))
}
