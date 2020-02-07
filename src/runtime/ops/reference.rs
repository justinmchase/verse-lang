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
  println!("ref: {:?}", val);
  match val {
    Some(v) => Ok(v.clone()),
    None => Err(InvalidReferenceError(name.to_string())),
  }
}

#[test]
fn reference_can_refer_to_var_in_scope() {
  let s = Scope::empty();
  s.add_var("x".to_string(), Value::Int(1));

  let r = reference(s, "x");
  assert_eq!(r, Ok(Value::Int(1)));
}

#[test]
fn reference_returns_error() {
  let s = Scope::empty();
  let r = reference(s, "x");
  assert_eq!(r, Err(InvalidReferenceError("x".to_string())));
}
