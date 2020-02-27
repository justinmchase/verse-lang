use std::rc::Rc;
use super::super::{
  Value,
  Context,
  RuntimeError,
};

pub fn string(_: Rc<Context>, s: String) -> Result<Value, RuntimeError> {
  Ok(Value::String(s))
}

#[test]
fn string_returns_string() {
  let c = Rc::new(Context::new());
  let r = string(c, "test".to_string());
  assert_eq!(r, Ok(Value::String("test".to_string())));
}
