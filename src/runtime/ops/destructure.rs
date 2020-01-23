use std::rc::Rc;
use super::super::{
  Scope,
  Value,
  exec,
  transform,
  RuntimeError,
  RuntimeError::{
    PatternNotMatchedError
  }
};
use super::super::super::ast::{
  Expression,
  Pattern,
};

pub fn destructure(start: Scope, pattern: &Pattern, expression: &Expression) -> Result<Value, RuntimeError> {
  match exec(start.clone(), &expression) {
    Ok(value) => {
      let args = Rc::new(vec![value]);
      let scope = Scope::new(args).with(start.vars);
      match transform(scope, pattern) {
        Ok(m) => {
          if m.matched {
            Ok(m.value)
          } else {
            Err(PatternNotMatchedError)
          }
        },
        Err(e) => Err(e)
      }
    },
    Err(e) => Err(e)
  }
}
