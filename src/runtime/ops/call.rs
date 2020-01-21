use std::collections::HashMap;
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
  Pattern::{Project}
};

pub fn call(scope: &mut Scope, func: &Expression, args: &Vec<Box<Expression>>) -> Result<Value, RuntimeError> {
  
  match exec(scope, &func) {
    Ok(value) => {
      match value {
        Function(p, e) => {
          let mut arguments = vec![];
          for a in args.iter() {
            match exec(scope, &a) {
              Ok(v) => arguments.push(v),
              Err(e) => return Err(e)
            }
          }
          let mut vars = HashMap::new();
          let mut scope = Scope::new(arguments, vars);
          transform(&mut scope, &Project(p, e))
        },
        _ => Err(NotCallableError(value)) // cannot call a non-function
      }

    },
    Err(e) => Err(e)
  }
}
