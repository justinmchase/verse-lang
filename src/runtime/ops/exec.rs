use std::rc::Rc;
use super::super::super::{
  runtime::{
    Value,
    Context,
    RuntimeError,
    ops::{
      add,
      array,
      block,
      call,
      destructure,
      function,
      int,
      none,
      reference,
      string,
      subtract,
    },
  },
  ast::{
    Expression
  }
};

pub fn exec(context: Rc<Context>, expr: &Expression) -> Result<Value, RuntimeError> {
  println!("  scope: {:?}", context);
  println!("     op: {:?}", expr);
  match expr {
    Expression::Add(l, r) => add(context, l, r),
    Expression::Block(e) => block(context, e),
    Expression::Call(v, args) => call(context, v, args),
    Expression::Destructure(p, e) => destructure(context, p, e),
    Expression::Int(i) => int(context, *i),
    Expression::None => none(context),
    Expression::Ref(name) => reference(context, name.to_string()),
    Expression::String(s) => string(context, s.to_string()),
    Expression::Sub(l, r) => subtract(context, l, r),
    Expression::Function(p, e) => function(context, p, e),
    Expression::Array(e) => array(context, e),
  }
}