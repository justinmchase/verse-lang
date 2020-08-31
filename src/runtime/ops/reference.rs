use super::super::{Context, RuntimeError, Value, Verse};
use std::rc::Rc;

pub fn reference(_: Rc<Verse>, context: Rc<Context>, name: String) -> Result<Value, RuntimeError> {
  let val = context.get_var(name.to_string());
  println!("    ref: {:?}", val);
  match val {
    Some(v) => Ok(v.clone()),
    None => Err(RuntimeError::InvalidReferenceError(name.to_string())),
  }
}

#[cfg(test)]
mod tests {
  use super::reference;
  use crate::runtime::{RuntimeError, Value, Verse};
  use std::rc::Rc;
  #[test]
  fn reference_can_refer_to_var_in_scope() {
    let v = Verse::default();
    let c = v.create_context();
    c.add_var(String::from("x").to_string(), Value::Int(1));

    let r = reference(Rc::new(v), c, String::from("x"));
    assert_eq!(r, Ok(Value::Int(1)));
  }

  #[test]
  fn reference_returns_error() {
    let v = Verse::default();
    let c = v.create_context();
    let r = reference(Rc::new(v), c, String::from("x"));
    assert_eq!(
      r,
      Err(RuntimeError::InvalidReferenceError(
        String::from("x").to_string()
      ))
    );
  }
}
