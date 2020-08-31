use super::super::{
  Scope,
  Match,
  RuntimeError,
};

pub fn default(start: Scope) -> Result<Match, RuntimeError> {
  Ok(Match::default(start))
}

#[cfg(test)]
mod tests {
  use super::default;
  use std::rc::Rc;
  use crate::runtime::{
    Value,
    Verse,
    Scope,
  };
  
  #[test]
  fn default_matches_as_none_always() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(1)]));
    let m = default(s);

    assert_eq!(m.clone().unwrap().value, Value::None);
    assert_eq!(m.unwrap().matched, true);
  }
}
