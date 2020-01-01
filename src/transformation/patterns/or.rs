use super::super::scope::Scope;
use super::super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::super::super::runtime::value::Value;
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
