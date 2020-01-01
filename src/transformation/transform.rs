use super::scope::Scope;
use super::pattern::Pattern;
use super::super::runtime::value::Value;
use super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::patterns::{
  value::value,
  range::range,
  quantity::quantity
};

pub fn or_pattern(scope: &mut Scope, patterns: Vec<Box<Pattern>>) -> Result<Value, TransformError> {
  for p in patterns.iter() {
    let r = transform(scope, *p.clone());
    match r {
      Err(e) => {},
      Ok(v) => {
        return Ok(v);
      }
    }
  }
  Err(Fail)
}

pub fn transform(scope: &mut Scope, pattern: Pattern) -> Result<Value, TransformError> {
  match pattern {
    Pattern::Default => Ok(Value::None),
    Pattern::Value(v) => value(scope, v),
    Pattern::Range(v0, v1) => range(scope, v0, v1),
    Pattern::Quantity(p, min, max) => quantity(scope, *p, min, max),
    Pattern::Or(patterns) => or_pattern(scope, patterns)
  }
}
