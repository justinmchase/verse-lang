use super::super::{
  Scope,
  Value,
  RuntimeError,
  RuntimeError::{
    InvalidReferenceError
  }
};

pub fn reference(scope: Scope, name: &str) -> Result<Value, RuntimeError> {
  let vars = (*scope.vars).borrow();
  let val = vars.get(name);
  println!("   - {:?}", val);
  match val {
    Some(v) => Ok(v.clone()),
    None => Err(InvalidReferenceError(name.to_string())),
  }
}