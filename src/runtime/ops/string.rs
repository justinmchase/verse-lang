use super::super::{Context, RuntimeError, Value, Verse};
use std::rc::Rc;

pub fn string(_: Rc<Verse>, _: Rc<Context>, s: String) -> Result<Value, RuntimeError> {
  Ok(Value::String(s))
}

#[cfg(test)]
mod tests {
  use super::string;
  use crate::runtime::{Value, Verse};
  use std::rc::Rc;
  #[test]
  fn string_returns_string() {
    let v = Verse::default();
    let c = v.create_context();
    let r = string(Rc::new(v), c, "test".to_string());
    assert_eq!(r, Ok(Value::String("test".to_string())));
  }
}
