use std::rc::Rc;
use super::super::{
  Value,
  RuntimeError,
  Context,
};

pub fn int(_: Rc<Context>, i: i32) -> Result<Value, RuntimeError> {
  Ok(Value::Int(i))
}

#[test]
fn int_returns_int() {
  let s = Rc::new(Context::new());
  let r = int(s, 0);
  assert_eq!(r, Ok(Value::Int(0)));
}
