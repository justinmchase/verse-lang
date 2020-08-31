use crate::ast::{
  Pattern
};
use crate::runtime::{
  Scope,
  Match,
  RuntimeError,
  transform,
};

pub fn not(start: Scope, pattern: &Pattern) -> Result<Match, RuntimeError> {
  match transform(start.clone(), pattern) {
    Ok(m) => {
      if m.matched {
        Ok(Match::fail(start))
      } else {
        Ok(Match::default(start))
      }
    },
    Err(e) => Err(e)
  }
}

#[cfg(test)]
mod tests {
  use super::not;
  use std::rc::Rc;
  use crate::ast::{
    Pattern,
  };
  use crate::runtime::{
    Value,
    Verse,
    Scope,
  };
  
  #[test]
  fn not_matches_empty() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![]));
    let p = Pattern::Any;
    let m = not(s, &p).unwrap();
    assert_eq!(m.matched, true);
    assert_eq!(m.value, Value::None);
  }

  #[test]
  fn not_fails_on_match() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(7)]));
    let p = Pattern::Any;
    let m = not(s, &p).unwrap();
    assert_eq!(m.matched, false);
    assert_eq!(m.value, Value::None);
  }

  #[test]
  fn not_succeeds_on_not_match() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(7)]));
    let p = Pattern::Equal(Value::Int(11));
    let m = not(s, &p).unwrap();
    assert_eq!(m.matched, true);
    assert_eq!(m.value, Value::None);
  }
}
