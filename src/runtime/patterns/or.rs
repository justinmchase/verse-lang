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

// Or matches the first pattern that matches against the same value
pub fn or(start: Scope, patterns: &Vec<Box<Pattern>>) -> Result<Match, RuntimeError> {
  let mut _match = Match::fail(start.clone());
  for p in patterns.iter() {
    match transform(start.clone(), p) {
      Ok(m) => {
        if m.matched || m.is_lr {
          return Ok(m);
        }
      },
      Err(e) => return Err(e)
    }
  }
  Ok(_match)
}

#[test]
fn or_matches_one() {
  let s = Scope::new(Rc::new(vec![Value::Int(0)]));
  let p = vec![Box::new(Pattern::Any)];
  let m = or(s, &p);

  assert_eq!(m.unwrap().value, Value::Int(0));
}

#[test]
fn or_second_pattern_can_match() {
  let s = Scope::new(Rc::new(vec![Value::Int(1)]));
  let p = vec![Box::new(Pattern::Equal(Value::Int(0))), Box::new(Pattern::Equal(Value::Int(1)))];
  let m = or(s, &p);

  assert_eq!(m.unwrap().value, Value::Int(1));
}

#[test]
fn or_fails_if_no_pattern_matches() {
  let s = Scope::new(Rc::new(vec![Value::Int(3)]));
  let p = vec![
    Box::new(Pattern::Equal(Value::Int(0))),
    Box::new(Pattern::Equal(Value::Int(1))),
    Box::new(Pattern::Equal(Value::Int(2))),
  ];
  let m = or(s, &p).unwrap();

  assert_eq!(m.matched, false);
}

#[test]
fn or_fails_if_not_enough_input() {
  let s = Scope::empty();
  let p = vec![Box::new(Pattern::Any)];
  let m = or(s, &p);

  assert_eq!(m.unwrap().matched, false);
}
