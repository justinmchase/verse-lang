use super::super::super::ast::{
  Pattern
};
use super::super::{
  Scope,
  Match,
  transform,
  RuntimeError
};

pub fn var(start: Scope, name: String, pattern: &Pattern) -> Result<Match, RuntimeError> {
  match transform(start.clone(), pattern) {
    Ok(m) => {
      if m.matched {
        Ok(Match::ok(m.value.clone(), start, m.end.add_var(name, m.value)))
      } else {
        Ok(m)
      }
    },
    Err(e) => Err(e)
  }
}