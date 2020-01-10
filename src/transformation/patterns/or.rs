use super::super::scope::Scope;
use super::super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::super::super::runtime::{Value};
use super::super::transform::transform;
use super::super::pattern::Pattern;

pub fn or(scope: &mut Scope, patterns: Vec<Box<Pattern>>) -> Result<Value, TransformError> {
  for p in patterns.iter() {
    let r = transform(scope, *p.clone());
    match r {
      Ok(v) => { return Ok(v); },
      _ => ()
    }
  }
  Err(Fail)
}

#[test]
fn eos() {
  let mut scope = Scope::new(vec![]);
  let res = or(&mut scope, vec![Box::new(Pattern::Value(Value::Int(0)))]);
  assert_eq!(res, Err(Fail));
}

#[test]
fn condition_fails() {
  let mut scope = Scope::new(vec![Value::Int(1)]);
  let res = or(&mut scope, vec![Box::new(Pattern::Value(Value::Int(0)))]);
  assert_eq!(res, Err(Fail));
}

#[test]
fn with_one_condition_success() {
  let mut scope = Scope::new(vec![Value::Int(0)]);
  let res = or(&mut scope, vec![Box::new(Pattern::Value(Value::Int(0)))]);
  assert_eq!(res, Ok(Value::Int(0)));
}

#[test]
fn with_multiple_conditions_success() {
  let mut scope = Scope::new(vec![Value::Int(1)]);
  let res = or(&mut scope, vec![
    Box::new(Pattern::Value(Value::Int(0))),
    Box::new(Pattern::Value(Value::Int(1)))
  ]);
  assert_eq!(res, Ok(Value::Int(1)));
}