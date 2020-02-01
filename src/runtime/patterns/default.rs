use std::rc::Rc;
use std::collections::HashMap;
use super::super::{
  Scope,
  Match,
  Value,
  RuntimeError,
  RuntimeError::{
    TransformError
  }
};

pub fn default(start: Scope) -> Result<Match, RuntimeError> {
  Ok(Match::default(start))
}

#[test]
fn default_matches_as_none_always() {
  let s = Scope::new(Rc::new(vec![Value::Int(1)]));
  let m = default(s);

  assert_eq!(m.unwrap().value, Value::None);
  assert_eq!(m.matched, true);
}