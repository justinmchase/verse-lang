use super::super::super::{
  runtime::{
    Scope,
    Value,
    RuntimeError,
    ops::{
      add,
      block,
      call,
      destructure,
      literal,
      reference,
      ret,
      subtract,
    },
  },
  ast::{
    Expression,
    Expression::{
      Add,
      Block,
      Call,
      Destructure,
      Literal,
      Ref,
      Return,
      Sub,
    }
  }
};

pub fn exec(scope: &mut Scope, expr: &Expression) -> Result<Value, RuntimeError> {
  println!(" op: {:?}", expr);
  match expr {
    Add(l, r) => add(scope, l, r),
    Block(e) => block(scope, e),
    Call(v, args) => call(scope, v, args),
    Destructure(p, e) => destructure(scope, p, e),
    Literal(v) => literal(scope, v),
    Ref(name) => reference(scope, name),
    Return(e) => ret(scope, e),
    Sub(l, r) => subtract(scope, l, r),
  }
}