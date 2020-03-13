use std::rc::Rc;
use super::super::super::ast::{
  Pattern
};
use super::super::{
  Scope,
  Match,
  Value,
  transform,
  RuntimeError
};

// And applies each rule to the same scope
pub fn quantity(start: Scope, pattern: &Pattern, min: &Option<usize>, max: &Option<usize>) -> Result<Match, RuntimeError> {
  let mut results = vec![];
  let mut last = start.clone();
  loop {
    match transform(last.clone(), pattern) {
      Ok(m) => {
        if !m.matched {
          break;
        } else {
          last = m.end;
          results.push(m.value);
          if max.is_some() && max.unwrap() >= results.len() {
            break;
          }
        }
      },
      Err(e) => return Err(e)
    }
  }

  let i = results.len();
  let j = min.or(Some(0)).unwrap();
  if i >= j && (max.is_none() || i <= max.unwrap()) {
    Ok(Match::ok(Value::Array(results), start, last))
  } else {
    Ok(Match::fail(start.clone()))
  }
}


#[test]
fn quantity_matches_empty() {
  let s = Scope::empty();
  let p = Pattern::Any;
  let m = quantity(s, &p, &None, &None).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Array(vec![]));
}

#[test]
fn quantity_matches_one() {
  let s = Scope::new(Rc::new(vec![Value::Int(0)]));
  let p = Pattern::Any;
  let m = quantity(s, &p, &None, &None).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Array(vec![Value::Int(0)]));
}

#[test]
fn quantity_exactly_matches_one() {
  let s = Scope::new(Rc::new(vec![Value::Int(0)]));
  let p = Pattern::Any;
  let m = quantity(s, &p, &Some(1), &Some(1)).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Array(vec![Value::Int(0)]));
}

#[test]
fn quantity_under_max_matches() {
  let s = Scope::new(Rc::new(vec![Value::Int(0)]));
  let p = Pattern::Any;
  let m = quantity(s, &p, &None, &Some(2)).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Array(vec![Value::Int(0)]));
}

#[test]
fn quantity_over_min_matches() {
  let s = Scope::new(Rc::new(vec![Value::Int(0)]));
  let p = Pattern::Any;
  let m = quantity(s, &p, &Some(0), &None).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Array(vec![Value::Int(0)]));
}

#[test]
fn quantity_under_min_fails() {
  let s = Scope::new(Rc::new(vec![Value::Int(0)]));
  let p = Pattern::Any;
  let m = quantity(s, &p, &Some(2), &None).unwrap();

  assert_eq!(m.matched, false);
}

#[test]
fn quantity_matches_min_zero() {
  let s = Scope::new(Rc::new(vec![Value::Int(1)]));
  let p = Pattern::Equal(Value::Int(0));
  let m = quantity(s, &p, &Some(0), &Some(1)).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Array(vec![]));
}

#[test]
fn quantity_matches_min_zero_max_one() {
  let s = Scope::new(Rc::new(vec![Value::Int(1), Value::Int(1)]));
  let p = Pattern::Equal(Value::Int(1));
  let m = quantity(s, &p, &Some(0), &Some(1)).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Array(vec![Value::Int(1)]));
}
