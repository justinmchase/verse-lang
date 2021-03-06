use super::super::super::ast::Pattern;
use super::super::{transform, Match, RuntimeError, Scope};

pub fn array(start: Scope, pattern: &Option<Box<Pattern>>) -> Result<Match, RuntimeError> {
  match start.next() {
    Some(next) => match next.step_into() {
      Some(s) => match pattern {
        Some(p) => match transform(s, p) {
          Ok(m) => match m.matched {
            true => Ok(Match::ok(next.value.clone(), start, next)),
            false => Ok(Match::fail(m.end)),
          },
          Err(e) => Err(e),
        },
        None => Ok(Match::ok(next.value, start, s)),
      },
      None => Err(RuntimeError::InvalidValueError(next.value)),
    },
    None => Err(RuntimeError::ScopeEmptyError),
  }
}

#[cfg(test)]
mod tests {
  use super::super::super::super::ast::Pattern;
  use super::super::super::{Scope, Value, Verse};
  use std::rc::Rc;

  #[test]
  fn array_matches_empty_array() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Array(vec![])]));
    let m = super::array(s, &None);
    let res = m.unwrap();
    assert_eq!(res.matched, true);
    assert_eq!(res.value, Value::Array(vec![]));
  }

  #[test]
  fn array_matches_non_empty_array() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let s = Scope::from(v, c, Rc::new(vec![Value::Array(vec![Value::Int(7)])]));
    let m = super::array(s, &Some(Box::new(Pattern::Any)));
    let res = m.unwrap();
    assert_eq!(res.matched, true);
    assert_eq!(res.value, Value::Array(vec![Value::Int(7)]));
  }

  #[test]
  fn array_fails_if_non_empty_array_doesnt_match_pattern() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Array(vec![Value::Int(7)])]));
    let m = super::array(s, &Some(Box::new(Pattern::Equal(Value::Int(11)))));
    let res = m.unwrap();
    assert_eq!(res.matched, false);
  }
}
