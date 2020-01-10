use super::super::scope::Scope;
use super::super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::super::super::runtime::{Value};
use super::super::transform::transform;
use super::super::pattern::Pattern;

pub fn quantity(scope: &mut Scope, pattern: Pattern, min: usize, max: Option<usize>) -> Result<Value, TransformError> {
  // Cannot successfully match any quantity against an empty stream
  if scope.peek() == None {
    return Err(Fail);
  }

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

#[test]
fn eos() {
  let mut scope = Scope::new(vec![]);
  let res = quantity(&mut scope, Pattern::Value(Value::Int(0)), 0, None);
  assert_eq!(res, Err(Fail));
}

#[test]
fn matches_none_successfully() {
  let mut scope = Scope::new(vec![Value::Int(1)]);
  let res = quantity(&mut scope, Pattern::Value(Value::Int(0)), 0, None);
  assert_eq!(res, Ok(Value::Array(vec![])));
}

#[test]
fn matches_exactly() {
  let mut scope = Scope::new(vec![Value::Int(0)]);
  let res = quantity(&mut scope, Pattern::Value(Value::Int(0)), 1, Some(1));
  assert_eq!(res, Ok(Value::Array(vec![Value::Int(0)])));
}

#[test]
fn matches_exactly_not_more() {
  let mut scope = Scope::new(vec![Value::Int(0), Value::Int(0)]);
  let res = quantity(&mut scope, Pattern::Value(Value::Int(0)), 1, Some(1));
  assert_eq!(res, Ok(Value::Array(vec![Value::Int(0)])));
}

#[test]
fn matches_multiple_with_min() {
  let mut scope = Scope::new(vec![Value::Int(0), Value::Int(0)]);
  let res = quantity(&mut scope, Pattern::Value(Value::Int(0)), 1, None);
  assert_eq!(res, Ok(Value::Array(vec![Value::Int(0), Value::Int(0)])));
}

#[test]
fn matches_max() {
  let mut scope = Scope::new(vec![Value::Int(0), Value::Int(0)]);
  let res = quantity(&mut scope, Pattern::Value(Value::Int(0)), 0, Some(1));
  assert_eq!(res, Ok(Value::Array(vec![Value::Int(0)])));
}

