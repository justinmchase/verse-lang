use crate::runtime::{Context, RuntimeError, Value, Verse};
use std::rc::Rc;

pub fn none(_: Rc<Verse>, _: Rc<Context>) -> Result<Value, RuntimeError> {
  Ok(Value::None)
}

#[cfg(test)]
mod tests {
  use super::none;
  use crate::runtime::{Value, Verse};
  use std::rc::Rc;
  #[test]
  fn none_returns_none() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let r = none(v, c);
    assert_eq!(r, Ok(Value::None));
  }
}
