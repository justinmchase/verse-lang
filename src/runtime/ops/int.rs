use super::super::{
  Scope,
  Value,
  RuntimeError,
};

pub fn int(_: Scope, i: i32) -> Result<Value, RuntimeError> {
  Ok(Value::Int(i))
}

#[test]
fn int_returns_int() {
  let s = Scope::empty();
  let r = int(s, 0);
  assert_eq!(r, Ok(Value::Int(0)));
}
