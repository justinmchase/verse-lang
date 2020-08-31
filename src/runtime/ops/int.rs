use super::super::{Context, RuntimeError, Value, Verse};
use std::rc::Rc;

pub fn int(_: Rc<Verse>, _: Rc<Context>, i: i32) -> Result<Value, RuntimeError> {
  Ok(Value::Int(i))
}

#[cfg(test)]
mod tests {
  use super::int;
  use crate::runtime::{Value, Verse};
  use std::rc::Rc;
  #[test]
  fn int_returns_int() {
    let v = Rc::new(Verse::default());
    let s = v.create_context();
    let r = int(v, s, 0);
    assert_eq!(r, Ok(Value::Int(0)));
  }
}
