use super::super::{Context, RuntimeError, Value, Verse};
use std::rc::Rc;

pub fn value(_: Rc<Verse>, _: Rc<Context>, v: &Value) -> Result<Value, RuntimeError> {
  Ok(v.clone())
}

#[cfg(test)]
mod tests {
  use super::value;
  use crate::runtime::{Value, Verse};
  use std::rc::Rc;
  #[test]
  fn int_returns_int() {
    let v = Verse::default();
    let c = v.create_context();
    let r = value(Rc::new(v), c, &Value::Int(7));
    assert_eq!(r, Ok(Value::Int(7)));
  }
}
