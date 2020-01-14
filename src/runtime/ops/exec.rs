use super::super::super::{
  runtime::{
    Scope,
    Value,
    RuntimeError,
    ops::{
      add,
      reference,
      subtract,
    },
  },
  ast::{
    Expression,
    Expression::{
      Add,
      Ref,
      Sub
    }
  }
};

pub fn exec(scope: &mut Scope, expr: &Expression) -> Result<Value, RuntimeError> {
  println!(" op: {:?}", expr);
  match expr {
    Ref(name) => reference(scope, name.to_string()),
    Add(l, r) => add(scope, l, r),
    Sub(l, r) => subtract(scope, l, r),
  }
}