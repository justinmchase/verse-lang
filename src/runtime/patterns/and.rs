use std::rc::Rc;
use std::collections::HashMap;
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

pub fn and(start: Scope, patterns: &Vec<Box<Pattern>>) -> Result<Match, RuntimeError> {
  let mut _match = Match::fail(start.clone());
  let mut results = vec![];
  for p in patterns.iter() {
    match transform(_match.end.clone(), p) {
      Ok(m) => {
        if !m.matched {
          return Ok(m);
        } else {
          let value = m.value.clone();
          results.push(value);
          _match = m;
        }
      },
      Err(e) => return Err(e)
    }
  }
  Ok(Match::ok(Value::Array(results), start, _match.end))
}

#[test]
fn and_matches_one() {
  let s = Scope::new(Rc::new(vec![Value::Int(0)]));
  let p = vec![Box::new(Pattern::Any)];
  let m = and(s, &p);

  assert_eq!(m.unwrap().value, Value::Array(vec![Value::Int(0)]));
}

#[test]
fn and_matches_two() {
  let s = Scope::new(Rc::new(vec![Value::Int(0), Value::Int(1)]));
  let p = vec![Box::new(Pattern::Any), Box::new(Pattern::Any)];
  let m = and(s, &p);

  assert_eq!(m.unwrap().value, Value::Array(vec![Value::Int(0), Value::Int(1)]));
}

#[test]
fn and_fails_if_not_enough_input() {
  let s = Scope::new(Rc::new(vec![Value::Int(0)]));
  let p = vec![Box::new(Pattern::Any), Box::new(Pattern::Any)];
  let m = and(s, &p);

  assert_eq!(m.unwrap().matched, false); 
}
