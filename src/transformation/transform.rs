use super::scope::Scope;
use super::pattern::Pattern;
use super::super::runtime::value::Value;
use super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::patterns::range::range;
use super::patterns::value::value;

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

pub fn slice_pattern(scope: &mut Scope, pattern: Pattern, min: usize, max: Option<usize>) -> Result<Value, TransformError> {
  let mut results: Vec<Value> = Vec::new();
  while true {
    let i = results.len();
    if (max != None && i >= max.unwrap()) {
      return Ok(Value::Array(results));
    }

    let r = transform(scope, pattern.clone());
    match r {
      Err(e) => {
        if i < min {
          return Err(Fail);
        } else {
          return Ok(Value::Array(results));
        }
      },
      Ok(v) => {
        results.push(v);
      }
    }
  }
  Err(Fail)
}

pub fn transform(scope: &mut Scope, pattern: Pattern) -> Result<Value, TransformError> {
  match pattern {
    Pattern::Default => Ok(Value::None),
    Pattern::Value(v) => value(scope, v),
    Pattern::Range((v0, v1)) => range(scope, v0, v1),
    Pattern::Slice((p, min, max)) => slice_pattern(scope, *p, min, max),
    Pattern::Or(patterns) => or_pattern(scope, patterns)
  }
}
