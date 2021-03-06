use super::super::{
  Scope,
  Match,
  RuntimeError,
};

pub fn any(start: Scope) -> Result<Match, RuntimeError> {
  let next = start.next();
  match next {
    Some(end) => Ok(Match::ok(end.value.clone(), start, end)),
    None => Ok(Match::fail(start)),
  }
}

#[cfg(test)]
mod tests {
  use super::any;
  use std::rc::Rc;
  use crate::runtime::{
    Value,
    Verse,
    Scope,
    Match,
  };
  
  #[test]
  fn any_matches_non_empty() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::None]));
    let m = any(s);

    assert_eq!(m.unwrap().value, Value::None);
  }

  #[test]
  fn any_fails_empty() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![]));
    let m = any(s.clone());

    assert_eq!(m, Ok(Match::fail(s)));
  }
}
