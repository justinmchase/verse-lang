use crate::ast::{
  Pattern
};
use crate::runtime::{
  Scope,
  Match,
  Value,
  transform,
  RuntimeError,
};

// Then applies each rule to sequential scopes
pub fn then(start: Scope, patterns: &Vec<Box<Pattern>>) -> Result<Match, RuntimeError> {
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

#[cfg(test)]
mod tests {
  use super::then;

  use std::rc::Rc;
  use crate::ast::{
    Pattern
  };
  use crate::runtime::{
    Value,
    Verse,
    Scope,
  };

  #[test]
  fn then_matches_one() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(0)]));
    let p = vec![Box::new(Pattern::Any)];
    let m = then(s, &p);

    assert_eq!(m.unwrap().value, Value::Array(vec![Value::Int(0)]));
  }

  #[test]
  fn then_matches_two() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(0), Value::Int(1)]));
    let p = vec![Box::new(Pattern::Any), Box::new(Pattern::Any)];
    let m = then(s, &p);

    assert_eq!(m.unwrap().value, Value::Array(vec![Value::Int(0), Value::Int(1)]));
  }

  #[test]
  fn then_fails_if_not_enough_input() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(0)]));
    let p = vec![Box::new(Pattern::Any), Box::new(Pattern::Any)];
    let m = then(s, &p);

    assert_eq!(m.unwrap().matched, false); 
  }
}
