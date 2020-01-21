use super::super::{
  Value,
  Scope,
  RuntimeError,
  RuntimeError::{
    TransformError
  }
};

pub fn any(scope: &mut Scope) -> Result<Value, RuntimeError> {
  let next = scope.next();
  match next {
    Some(v) => Ok(v.clone()),
    None => Err(TransformError)
  }
}