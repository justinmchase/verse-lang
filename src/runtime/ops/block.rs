use std::rc::Rc;
use super::super::{
  Value,
  Context,
  exec,
  RuntimeError,
};
use super::super::super::ast::{
  Expression,
  Pattern,
};

pub fn block(context: Rc<Context>, expressions: &Vec<Box<Expression>>) -> Result<Value, RuntimeError> {
  // todo: should handle the return case..
  let mut last = Value::None;
  for expression in expressions.iter() {
    match exec(context.clone(), &expression) {
      Err(e) => return Err(e),
      Ok(v) => { last = v; }
    }
  }
  Ok(last)
}

#[test]
fn block_with_expr_succeeds() {
  let e = vec![
    Box::new(Expression::Int(1))
  ];
  let c = Rc::new(Context::new());
  let r = block(c, &e);
  assert_eq!(r, Ok(Value::Int(1)));
}

#[test]
fn block_with_no_expr_returns_none() {
  let e = vec![];
  let c = Rc::new(Context::new());
  let r = block(c, &e);
  assert_eq!(r, Ok(Value::None));
}

#[test]
fn block_last_expression_returns() {
  let e = vec![
    Box::new(Expression::Int(1)),
    Box::new(Expression::Int(2))
  ];
  let c = Rc::new(Context::new());
  let r = block(c, &e);
  assert_eq!(r, Ok(Value::Int(2)));
}

#[test]
fn block_expressions_share_scope() {
  let c = Rc::new(Context::new());
  let e = vec![
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
      Box::new(Expression::Int(7))
    )),
    Box::new(Expression::Ref(String::from("x")))
  ];
  let r = block(c, &e);
  assert_eq!(r, Ok(Value::Int(7)));
}