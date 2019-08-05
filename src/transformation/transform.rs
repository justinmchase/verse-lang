use super::scope::Scope;
use super::pattern::Pattern;
use super::super::runtime::value::Value;
use super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::patterns::range::range;
use super::patterns::value::value;

pub fn slice_pattern(scope: &mut Scope, pattern: Pattern, min: usize, max: Option<usize>) -> Result<Value, TransformError> {
  // let mut results: Vec<Value> = Vec::new();
  // while true {
  //   let i = results.len();
  //   let r = transform(scope, pattern.clone());
  //   match r {
  //     Err(e) => {
  //       if min < i && (max == None || i < max.unwrap()) {

  //       }
  //     },
  //     Ok(v) => {
  //       results.push(v);
  //     }
  //   }
  //   if (max == None || results.len() >= max) {

  //   }
  // }
  Err(Fail)
}

pub fn transform(scope: &mut Scope, pattern: Pattern) -> Result<Value, TransformError> {
  match pattern {
    Pattern::Default => Ok(Value::None),
    Pattern::Value(v) => value(scope, v),
    Pattern::Range((v0, v1)) => range(scope, v0, v1),
    Pattern::Slice((p, min, max)) => slice_pattern(scope, *p, min, max),
  }
}
