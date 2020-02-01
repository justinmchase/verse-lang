use std::rc::Rc;
use super::super::super::ast::{
  Expression,
  Pattern,
};
use super::super::{
  Scope,
  Match,
  Value,
  RuntimeError,
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

#[test]
fn project_success() {
  let p = Pattern::Any;
  let e = Expression::Literal(Value::Int(7));
  let s = Scope::new(Rc::new(vec![Value::Int(11)]));

  let r = project(s, &p, &e);
  assert_eq!(r.unwrap().value, Value::Int(7));
}

#[test]
fn project_expr_can_access_vars() {
  let p = Pattern::Var("x", Box::new(Pattern::Any));
  let e = Expression::Ref("x");
  let s = Scope::new(Rc::new(vec![Value::Int(7)]));

  let r = project(s, &p, &e);
  assert_eq!(r.unwrap().value, Value::Int(7));
}