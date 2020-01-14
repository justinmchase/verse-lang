use super::super::{
  Value,
  Scope,
  RuntimeError,
  RuntimeError::{
    TransformError
  }
};

pub fn any(scope: &mut Scope) -> Result<Value, RuntimeError> {
  match scope.next() {
    Some(v) => Ok(v.clone()),
    None => Err(TransformError)
  }
}