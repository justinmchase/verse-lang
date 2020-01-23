use std::rc::Rc;
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
    NotCallableError,
    PatternNotMatchedError
  }
};
use super::super::super::ast::{
  Expression,
  Pattern::{Project}
};

pub fn call(start: Scope, func: &Expression, args: &Vec<Box<Expression>>) -> Result<Value, RuntimeError> {
  match exec(start.clone(), &func) {
    Ok(value) => {
      match value {
        Function(p, e) => {
          let mut arguments = vec![];
          for a in args.iter() {
            match exec(start.clone(), &a) {
              Ok(v) => arguments.push(v),
              Err(e) => { return Err(e) }
            }
          }
          let scope = Scope::new(Rc::new(arguments));
          match transform(scope, &Project(p, e)) {
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
        _ => Err(NotCallableError(value)) // cannot call a non-function
      }
    },
    Err(e) => Err(e)
  }
}
