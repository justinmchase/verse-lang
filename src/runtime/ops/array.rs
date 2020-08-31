use std::rc::Rc;
use super::super::{
  Value,
  Verse,
  Context,
  RuntimeError,
  exec
};
use super::super::super::ast::{
  Expression
};

pub fn array(verse: Rc<Verse>, context: Rc<Context>, exp: &Vec<Box<Expression>>) -> Result<Value, RuntimeError> {
  let mut values = vec![];
  for e in exp.iter() {
    match exec(verse.clone(), context.clone(), e) {
      Ok(v) => {
        values.push(v);
      },
      Err(e) => {
        return Err(e)
      }
    }
  }

  Ok(Value::Array(values))
}
