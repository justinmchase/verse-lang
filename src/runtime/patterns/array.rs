use std::rc::Rc;
use super::super::super::ast::{
  Pattern
};
use super::super::{
  Value,
  Scope,
  Match,
  transform,
  RuntimeError
};

pub fn array(start: Scope, pattern: &Option<Box<Pattern>>) -> Result<Match, RuntimeError> {
  match start.next() {
    Some(n) => {
      let next = n.clone();
      match n.value {
        Value::Array(items) => {
          match pattern {
            Some(p) => {
              let args = Rc::new(items.to_vec());
              let s = Scope::new(args).with_vars(n.vars);
              match transform(s, p) {
                Ok(_m) => Ok(Match::ok(Value::Array(items), start, next)),
                Err(e) => Err(e)
              }
            },
            None => Ok(Match::ok(Value::Array(items), start, next))
          }
        },
        _ => Err(RuntimeError::InvalidValueError(n.value))
      }
    },
    None => Err(RuntimeError::ScopeEmptyError)
  }
}

#[test]
fn array_matches_empty_array() {
  let s = Scope::new(Rc::new(vec![Value::Array(vec![])]));
  let m = array(s, &None);

  let res = m.unwrap();

  assert_eq!(res.matched, true);
  assert_eq!(res.value, Value::Array(vec![]));
}

#[test]
fn array_matches_non_empty_array() {
  let s = Scope::new(Rc::new(vec![Value::Array(vec![Value::Int(7)])]));
  let m = array(s, &Some(Box::new(Pattern::Any)));

  let res = m.unwrap();

  assert_eq!(res.matched, true);
  assert_eq!(res.value, Value::Array(vec![Value::Int(7)]));
}