use super::super::super::ast::Expression;
use super::super::{exec, Context, RuntimeError, Value, Verse};
use std::rc::Rc;

pub fn subtract(
  verse: Rc<Verse>,
  context: Rc<Context>,
  left: &Expression,
  right: &Expression,
) -> Result<Value, RuntimeError> {
  let r0 = exec(verse.clone(), context.clone(), left);
  if r0.is_err() {
    return r0;
  }
  let r1 = exec(verse.clone(), context.clone(), right);
  if r1.is_err() {
    return r1;
  }

  let l = r0.ok().unwrap();
  let r = r1.ok().unwrap();
  match l {
    Value::Int(li32) => match r {
      Value::Int(ri32) => Ok(Value::Int(li32 - ri32)),
      _ => Err(RuntimeError::InvalidValueError(r)),
    },
    _ => Err(RuntimeError::InvalidValueError(l)),
  }
}

#[cfg(test)]
mod tests {
  use super::subtract;
  use crate::ast::Expression;
  use crate::runtime::{RuntimeError, Value, Verse};
  use std::rc::Rc;
  #[test]
  fn sub_tests() {
    let verse = Rc::new(Verse::default());
    let values = vec![
      (Expression::Int(3), Expression::Int(2), Ok(Value::Int(1))),
      (
        Expression::String("a".to_string()),
        Expression::String("b".to_string()),
        Err(RuntimeError::InvalidValueError(Value::String(
          "a".to_string(),
        ))),
      ),
      (
        Expression::String("a".to_string()),
        Expression::Int(0),
        Err(RuntimeError::InvalidValueError(Value::String(
          "a".to_string(),
        ))),
      ),
      (
        Expression::Int(0),
        Expression::String("a".to_string()),
        Err(RuntimeError::InvalidValueError(Value::String(
          "a".to_string(),
        ))),
      ),
      (
        Expression::None,
        Expression::None,
        Err(RuntimeError::InvalidValueError(Value::None)),
      ),
      // tood: all combinations...
    ];
    for (l, r, v) in values.iter() {
      let c = verse.create_context();
      let res = subtract(verse.clone(), c, l, r);
      assert_eq!(res, *v);
    }
  }
}
