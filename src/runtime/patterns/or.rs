use crate::ast::{
  Pattern
};
use crate::runtime::{
  Scope,
  Match,
  transform,
  RuntimeError,
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

#[cfg(test)]
mod tests {
  use super::or;
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
  fn or_matches_one() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(0)]));
    let p = vec![Box::new(Pattern::Any)];
    let m = or(s, &p);

    assert_eq!(m.unwrap().value, Value::Int(0));
  }

  #[test]
  fn or_second_pattern_can_match() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(1)]));
    let p = vec![Box::new(Pattern::Equal(Value::Int(0))), Box::new(Pattern::Equal(Value::Int(1)))];
    let m = or(s, &p);

    assert_eq!(m.unwrap().value, Value::Int(1));
  }

  #[test]
  fn or_fails_if_no_pattern_matches() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(3)]));
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
    let v = Verse::default();
    let s = Scope::empty(Rc::new(v));
    let p = vec![Box::new(Pattern::Any)];
    let m = or(s, &p);

    assert_eq!(m.unwrap().matched, false);
  }
}
