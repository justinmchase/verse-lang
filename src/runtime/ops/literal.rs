use super::super::{
  Scope,
  Value,
  RuntimeError,
  RuntimeError::{
    InvalidReferenceError
  }
};

pub fn literal(_scope: &mut Scope, value: &Value) -> Result<Value, RuntimeError> {
  Ok(value.clone())
}