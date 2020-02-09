use std::rc::Rc;
use super::super::{
  Scope,
  Match,
  Value,
  RuntimeError,
  value_eq
};

pub fn equal(start: Scope, v: &Value) -> Result<Match, RuntimeError> {
  let next = start.next();
  match next {
    Some(end) => {
      let ev = end.value.clone();
      if value_eq(&ev, &v) {
        Ok(Match::ok(ev, start, end))
      } else {
        Ok(Match::fail(start))
      }
    },
    None => Ok(Match::fail(start)),
  }
}

#[test]
fn equal_does_not_match_empty() {
  let s = Scope::new(Rc::new(vec![]));
  let m = equal(s, &Value::Int(0)).unwrap();

  assert_eq!(m.matched, false);
}

#[test]
fn equal_matches_correct_value() {
  let s = Scope::new(Rc::new(vec![Value::Int(7)]));
  let m = equal(s.clone(), &Value::Int(7)).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Int(7));
}

#[test]
fn equal_does_not_match_wrong_value() {
  let s = Scope::new(Rc::new(vec![Value::Int(7)]));
  let m = equal(s.clone(), &Value::Int(11)).unwrap();

  assert_eq!(m.matched, false);
}
