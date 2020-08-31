use std::rc::Rc;
use super::super::{
  Value,
  Verse,
  Function,
  NativeFunction,
  NativeFunctionHandler,
  Context,
  RuntimeError,
};
use super::super::super::ast::{
  Expression,
  Pattern
};

pub fn function(_: Rc<Verse>, context: Rc<Context>, p: &Pattern, e: &Option<Expression>) -> Result<Value, RuntimeError> {
  Ok(Value::Function(
    Box::new(Function::new(p, e)),
    // verse?
    context
  ))
}

pub fn native_function(_: Rc<Verse>, context: Rc<Context>, p: &Pattern, f: &NativeFunctionHandler) -> Result<Value, RuntimeError> {
  Ok(Value::NativeFunction(
    Box::new(NativeFunction::new(p, f)),
    // verse?
    context
  ))
}
