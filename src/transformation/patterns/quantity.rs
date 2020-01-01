use super::super::scope::Scope;
use super::super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::super::super::runtime::value::Value;
use super::super::transform::transform;
use super::super::pattern::Pattern;

pub fn quantity(scope: &mut Scope, pattern: Pattern, min: usize, max: Option<usize>) -> Result<Value, TransformError> {
  let mut results: Vec<Value> = Vec::new();
  let mut pos = scope.pos();
  loop {
    let i = results.len();
    match max {
      Some(m) => if i >= m { break },
      _ => ()
    }

    let r = transform(scope, pattern.clone());
    match r {
      Ok(v) => results.push(v),
      _ => break
    }
    
    // If we aren't progressing the match then also break
    // This prevents infinite looping on say `default*`
    let p = scope.pos();
    if p <= pos {
      break
    } else {
      pos = p;
    }
  }
  
  let i = results.len();
  if i >= min && (max == None || i <= max.unwrap()) {
    Ok(Value::Array(results))
  } else {
    Err(Fail)
  }
}
