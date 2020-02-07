use std::rc::Rc;
use super::super::{
  Scope,
  Value,
  exec,
  RuntimeError,
};
use super::super::super::ast::{
  Expression,
  Pattern,
};

pub fn block(scope: Scope, expressions: &Vec<Box<Expression>>) -> Result<Value, RuntimeError> {
  // todo: should handle the return case..
  let mut last = Value::None;
  for expression in expressions.iter() {
    match exec(scope.clone(), &expression) {
      Err(e) => return Err(e),
      Ok(v) => { last = v; }
    }
  }
  Ok(last)
}

#[test]
fn block_with_expr_succeeds() {
  let e = vec![
    Box::new(Expression::Literal(Value::Int(1)))
  ];
  let s = Scope::new(Rc::new(vec![]));
  let r = block(s, &e);
  assert_eq!(r, Ok(Value::Int(1)));
}

#[test]
fn block_with_no_expr_returns_none() {
  let e = vec![];
  let s = Scope::new(Rc::new(vec![]));
  let r = block(s, &e);
  assert_eq!(r, Ok(Value::None));
}

#[test]
fn block_last_expression_returns() {
  let e = vec![
    Box::new(Expression::Literal(Value::Int(1))),
    Box::new(Expression::Literal(Value::Int(2)))
  ];
  let s = Scope::new(Rc::new(vec![]));
  let r = block(s, &e);
  assert_eq!(r, Ok(Value::Int(2)));
}

#[test]
fn block_expressions_share_scope() {
  let s = Scope::empty();
  let e = vec![
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var("x", Box::new(Pattern::Any))),
      Box::new(Expression::Literal(Value::Int(7)))
    )),
    Box::new(Expression::Ref("x"))
  ];
  let r = block(s, &e);
  assert_eq!(r, Ok(Value::Int(7)));
}