use super::super::{
  Scope,
  Value,
  RuntimeError,
};

pub fn literal(_scope: Scope, value: &Value) -> Result<Value, RuntimeError> {
  Ok(value.clone())
}