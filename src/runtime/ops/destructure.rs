use super::super::{
  Scope,
  Value,
  Value::{
    Function
  },
  exec,
  transform,
  RuntimeError,
  RuntimeError::{
    InvalidValueError,
    NotCallableError
  }
};
use super::super::super::ast::{
  Expression,
  Pattern,
};

pub fn destructure(scope: &mut Scope, pattern: &Pattern, expression: &Expression) -> Result<Value, RuntimeError> {
  match exec(scope, &expression) {
    Ok(value) => {
      let mut scope = Scope::new(vec![value], scope.vars.clone());
      transform(&mut scope, pattern)
    },
    Err(e) => Err(e)
  }
}
