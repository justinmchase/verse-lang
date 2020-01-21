use super::super::{
  Scope,
  Value,
  exec,
  RuntimeError,
  RuntimeError::{
    InvalidValueError
  }
};
use super::super::super::ast::Expression;

pub fn ret(scope: &mut Scope, exp: &Expression) -> Result<Value, RuntimeError> {

  // todo: should unwind the current scope
  exec(scope, exp)
}
