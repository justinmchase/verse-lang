use crate::ast::{Expression, Pattern};
use crate::runtime::{exec, transform, Match, RuntimeError, Scope};

pub fn project(
  start: Scope,
  pattern: &Pattern,
  expression: &Option<Expression>,
) -> Result<Match, RuntimeError> {
  let verse = start.verse.clone();
  let ctx = start.context.clone();
  match transform(start.clone(), pattern) {
    Ok(m) => match m.matched {
      true => match expression {
        Some(e) => match exec(verse, ctx, e) {
          Ok(v) => Ok(Match::ok(v, start, m.end)),
          Err(e) => Err(e),
        },
        None => Ok(m),
      },
      false => Ok(m),
    },
    Err(e) => Err(e),
  }
}

#[cfg(test)]
mod tests {
  use super::project;
  use crate::ast::{Expression, Pattern};
  use crate::runtime::{Scope, Value, Verse};
  use std::rc::Rc;
  #[test]
  fn project_success() {
    let v = Verse::default();
    let p = Pattern::Any;
    let e = Box::new(Some(Expression::Int(7)));
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(11)]));

    let r = project(s, &p, &e);
    assert_eq!(r.unwrap().value, Value::Int(7));
  }

  #[test]
  fn project_expr_can_access_vars() {
    let v = Verse::default();
    let p = Pattern::Var(String::from("x"), Box::new(Pattern::Any));
    let e = Box::new(Some(Expression::Ref(String::from("x"))));
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(7)]));

    let r = project(s, &p, &e);
    assert_eq!(r.unwrap().value, Value::Int(7));
  }

  #[test]
  fn project_returns_match_without_expression() {
    let p = Pattern::Any;
    let e = None;
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(7)]));

    let r = project(s, &p, &e);
    assert_eq!(r.unwrap().value, Value::Int(7));
  }
}
