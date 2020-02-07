use super::super::{
  Scope,
  Value,
  RuntimeError,
};

pub fn literal(_: Scope, value: &Value) -> Result<Value, RuntimeError> {
  Ok(value.clone())
}

#[test]
fn literal_returns_value() {
  let s = Scope::empty();
  let r = literal(s, &Value::Int(1));
  assert_eq!(r, Ok(Value::Int(1)));
}
