use super::super::{
  Scope,
  Value,
  RuntimeError,
};

pub fn none(_: Scope) -> Result<Value, RuntimeError> {
  Ok(Value::None)
}

#[test]
fn none_returns_none() {
  let s = Scope::empty();
  let r = none(s);
  assert_eq!(r, Ok(Value::None));
}
