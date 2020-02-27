
use super::super::super::ast::Pattern;
use super::super::{
  Scope,
  Match,
  transform,
  RuntimeError
};

pub fn array(start: Scope, pattern: &Option<Box<Pattern>>) -> Result<Match, RuntimeError> {
  println!("1");
  match start.next() {
    Some(next) => {
      println!("2");
      match next.step_into() {
        Some(s) => {
          println!("3");
          match pattern {
            Some(p) => match transform(s, p) {
              Ok(m) => {
                println!("4");
                match m.matched {
                  true => Ok(Match::ok(next.value.clone(), start, next)),
                  false => Ok(Match::fail(m.end)),
                }
              },
              Err(e) => Err(e)
            },
            None => Ok(Match::ok(next.value, start, s))
          }
        },
        None => Err(RuntimeError::InvalidValueError(next.value))
      }
    },
    None => Err(RuntimeError::ScopeEmptyError)
  }
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;
  use super::super::super::super::ast::Pattern;
  use super::super::super::{
    Value,
    Scope,
    Context,
  };

  #[test]
  fn array_matches_empty_array() {
    let s = Scope::new(Rc::new(vec![Value::Array(vec![])]));
    let m = super::array(s, &None);
    let res = m.unwrap();
    assert_eq!(res.matched, true);
    assert_eq!(res.value, Value::Array(vec![]));
  }

  #[test]
  fn array_matches_non_empty_array() {
    let c = Rc::new(Context::new());
    let s = Scope::from(Rc::new(vec![Value::Array(vec![Value::Int(7)])]), c);
    let m = super::array(s, &Some(Box::new(Pattern::Any)));
    let res = m.unwrap();
    assert_eq!(res.matched, true);
    assert_eq!(res.value, Value::Array(vec![Value::Int(7)]));
  }

  #[test]
  fn array_fails_if_non_empty_array_doesnt_match_pattern() {
    let s = Scope::new(Rc::new(vec![Value::Array(vec![Value::Int(7)])]));
    let m = super::array(s, &Some(Box::new(Pattern::Equal(Value::Int(11)))));
    let res = m.unwrap();
    assert_eq!(res.matched, false);
  }
}