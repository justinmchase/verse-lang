use std::rc::Rc;
use super::super::{
  Value,
  Function,
  Context,
  RuntimeError,
};
use super::super::super::ast::{
  Expression,
  Pattern
};

pub fn function(context: Rc<Context>, p: &Pattern, e: &Option<Expression>) -> Result<Value, RuntimeError> {
  Ok(Value::Function(
    Box::new(Function::new(p, e)),
    context
  ))
}
