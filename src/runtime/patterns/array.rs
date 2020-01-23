use std::rc::Rc;
use super::super::super::ast::{
  Pattern
};
use super::super::{
  Value,
  Scope,
  Match,
  transform,
  RuntimeError,
  RuntimeError::{
    TransformError
  }
};

pub fn array(start: Scope, pattern: &Pattern) -> Result<Match, RuntimeError> {
  match start.next() {
    Some(next) => match next.value {
      Value::Array(items) => {
        let args = Rc::new(items.to_vec());
        let s = Scope::new(args).with(next.vars);
        transform(s, pattern)
      },
      _ => Err(TransformError) // todo: implement tuple
    }
    None => Err(TransformError)
  }
}