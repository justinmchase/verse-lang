use super::super::super::{
  runtime::{
    Scope,
    Value,
    RuntimeError,
    ops::{
      add,
      array,
      block,
      call,
      destructure,
      function,
      int,
      literal,
      none,
      reference,
      subtract,
    },
  },
  ast::{
    Expression
  }
};

pub fn exec(scope: Scope, expr: &Expression) -> Result<Value, RuntimeError> {
  println!(" op: {:?}", expr);
  match expr {
    Expression::Add(l, r) => add(scope, l, r),
    Expression::Block(e) => block(scope, e),
    Expression::Call(v, args) => call(scope, v, args),
    Expression::Destructure(p, e) => destructure(scope, p, e),
    Expression::Int(i) => int(scope, *i),
    Expression::Literal(v) => literal(scope, v),
    Expression::None => none(scope),
    Expression::Ref(name) => reference(scope, name),
    Expression::Sub(l, r) => subtract(scope, l, r),
    Expression::Function(p, e) => function(scope, p, e),
    Expression::Array(e) => array(scope, e),
  }
}