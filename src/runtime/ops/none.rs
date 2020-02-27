use std::rc::Rc;
use super::super::{
  Value,
  RuntimeError,
  Context,
};

pub fn none(_: Rc<Context>) -> Result<Value, RuntimeError> {
  Ok(Value::None)
}

#[test]
fn none_returns_none() {
  let s = Rc::new(Context::new());
  let r = none(s);
  assert_eq!(r, Ok(Value::None));
}
