use super::super::{
  Scope,
  Value,
  RuntimeError,
};

pub fn string(_: Scope, s: String) -> Result<Value, RuntimeError> {
  Ok(Value::String(s))
}

#[test]
fn string_returns_string() {
  let s = Scope::empty();
  let r = string(s, "test".to_string());
  assert_eq!(r, Ok(Value::String("test".to_string())));
}
