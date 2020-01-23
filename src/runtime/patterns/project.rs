use super::super::super::ast::{
  Expression,
  Pattern,
};
use super::super::{
  Scope,
  Match,
  RuntimeError,
  RuntimeError::{
    InvalidValueError
  },
  transform,
  exec,
};

pub fn project(start: Scope, pattern: &Pattern, expression: &Expression) -> Result<Match, RuntimeError> {
  match transform(start.clone(), pattern) {
    Ok(m) => match exec(m.end.clone(), expression) {
      Ok(v) => Ok(Match::ok(v, start, m.end)),
      Err(e) => Err(e)
    },
    Err(e) => Err(e)
  }
}