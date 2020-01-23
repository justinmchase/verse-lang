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
  let mut end = Match::fail(start.clone());
  let mut results = vec![];
  for p in patterns.iter() {
    match transform(start.clone(), p) {
      Ok(m) => {
        if !m.matched {
          return Ok(m);
        } else {
          let value = m.value.clone();
          results.push(value);
          end = m;
        }
      },
      Err(e) => return Err(e)
    }
  }
  Ok(Match::ok(Value::Array(results), start, end.end))
}

#[test]
fn and_matches_one() {
  let s = Scope::new(Rc::new(vec![Value::None]));
  let p = vec![Box::new(Pattern::Any)];
  let m = and(s, &p);

  assert_eq!(m.unwrap().value, Value::Array(vec![Value::None]));
}

#[test]
fn and_matches_two() {
  let s = Scope::new(Rc::new(vec![Value::None, Value::None]));
  let p = vec![Box::new(Pattern::Any), Box::new(Pattern::Any)];
  let m = and(s, &p);

  assert_eq!(m.unwrap().value, Value::Array(vec![Value::None, Value::None]));
}
