use crate::ast::Expression;
use crate::runtime::{exec, Context, RuntimeError, Value, Verse};
use std::rc::Rc;

pub fn block(
  verse: Rc<Verse>,
  context: Rc<Context>,
  expressions: &Vec<Box<Expression>>,
) -> Result<Value, RuntimeError> {
  // todo: should handle the return case..
  let mut last = Value::None;
  for expression in expressions.iter() {
    match exec(verse.clone(), context.clone(), &expression) {
      Err(e) => return Err(e),
      Ok(v) => {
        last = v;
      }
    }
  }
  Ok(last)
}

#[cfg(test)]
mod tests {
  use super::block;
  use crate::ast::{Expression, Pattern};
  use crate::runtime::{Value, Verse};
  use std::rc::Rc;

  #[test]
  fn block_with_expr_succeeds() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let e = vec![Box::new(Expression::Int(1))];
    let r = block(v, c, &e);
    assert_eq!(r, Ok(Value::Int(1)));
  }

  #[test]
  fn block_with_no_expr_returns_none() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let e = vec![];
    let r = block(v, c, &e);
    assert_eq!(r, Ok(Value::None));
  }

  #[test]
  fn block_last_expression_returns() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let e = vec![Box::new(Expression::Int(1)), Box::new(Expression::Int(2))];
    let r = block(v, c, &e);
    assert_eq!(r, Ok(Value::Int(2)));
  }

  #[test]
  fn block_expressions_share_scope() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let e = vec![
      Box::new(Expression::Destructure(
        Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
        Box::new(Expression::Int(7)),
      )),
      Box::new(Expression::Ref(String::from("x"))),
    ];
    let r = block(v, c, &e);
    assert_eq!(r, Ok(Value::Int(7)));
  }
}
