use super::super::scope::Scope;
use super::super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::super::super::runtime::{Value};

pub fn default(scope: &mut Scope) -> Result<Value, TransformError> {
  match scope.peek() {
    None => Err(Fail),
    _ => Ok(Value::None)
  }
}

#[test]
fn eos() {
  let mut scope = Scope::new(vec![]);
  let res = default(&mut scope);
  assert_eq!(res, Err(Fail));  
}

#[test]
fn has_content() {
  let mut scope = Scope::new(vec![Value::Int(0)]);
  let res = default(&mut scope);
  assert_eq!(res, Ok(Value::None));
}
