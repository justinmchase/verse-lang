use super::super::{
  Scope,
  Value,
  RuntimeError,
  RuntimeError::{
    InvalidReferenceError
  }
};

pub fn reference(scope: &mut Scope, name: &str) -> Result<Value, RuntimeError> {
  let val = scope.vars.get(name);
  println!("   - {:?}", val);
  match val {
    Some(v) => Ok(v.clone()),
    None => Err(InvalidReferenceError(name.to_string())),
  }
}