use std::rc::Rc;
use crate::runtime::{
  Context,
  Value,
  RuntimeError,
};
use crate::ast::{
  Expression,
  Pattern,
};

pub type NativeFunctionHandler = fn(context: Rc<Context>) -> Result<Value, RuntimeError>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct NativeFunction {
  pub pattern: Pattern,
  pub handler: NativeFunctionHandler,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Function {
  pub pattern: Pattern,
  pub expression: Option<Expression>,
}

impl NativeFunction {
  pub fn new(pattern: &Pattern, handler: &NativeFunctionHandler) -> Self {
    NativeFunction {
      pattern: pattern.clone(),
      handler: *handler,
    }
  }
}

impl Function {
  pub fn new(pattern: &Pattern, expression: &Option<Expression>) -> Self {
    Function {
      pattern: pattern.clone(),
      expression: expression.clone(),
    }
  }

  pub fn default() -> Self {
    Function {
      pattern: Pattern::Default,
      expression: None,
    }
  }
}
