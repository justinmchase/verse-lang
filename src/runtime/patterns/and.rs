use super::super::super::ast::{
  Pattern
};
use super::super::{
  Scope,
  Match,
  transform,
  RuntimeError,
};

// And applies each rule to the same scope
pub fn and(start: Scope, patterns: &Vec<Box<Pattern>>) -> Result<Match, RuntimeError> {
  let mut _match = Match::fail(start.clone());
  for p in patterns.iter() {
    match transform(start.clone(), p) {
      Ok(m) => {
        if !m.matched {
          return Ok(m);
        } else {
          _match = m;
        }
      },
      Err(e) => return Err(e)
    }
  }
  Ok(_match)
}

#[cfg(test)]
mod tests {
  use super::and;
  
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
  fn and_matches_one() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(0)]));
    let p = vec![Box::new(Pattern::Any)];
    let m = and(s, &p);

    assert_eq!(m.unwrap().value, Value::Int(0));
  }

  #[test]
  fn and_matches_same_value_twice() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(0), Value::Int(1)]));
    let p = vec![Box::new(Pattern::Any), Box::new(Pattern::Any)];
    let m = and(s, &p);

    assert_eq!(m.unwrap().value, Value::Int(0));
  }

  #[test]
  fn and_fails_if_not_enough_input() {
    let v = Verse::default();
    let s = Scope::empty(Rc::new(v));
    let p = vec![Box::new(Pattern::Any)];
    let m = and(s, &p);

    assert_eq!(m.unwrap().matched, false);
  }
}
