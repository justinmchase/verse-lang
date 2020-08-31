use crate::ast::{Expression, Pattern};
use crate::runtime::{exec, transform, Context, RuntimeError, Scope, Value, Verse};
use std::rc::Rc;

pub fn destructure(
  verse: Rc<Verse>,
  context: Rc<Context>,
  pattern: &Pattern,
  expression: &Expression,
) -> Result<Value, RuntimeError> {
  match exec(verse.clone(), context.clone(), &expression) {
    Ok(value) => {
      let args = Rc::new(vec![value]);
      let scope = Scope::from(verse, context, args);
      match transform(scope, pattern) {
        Ok(m) => {
          if m.matched {
            Ok(m.value)
          } else {
            Err(RuntimeError::PatternNotMatchedError)
          }
        }
        Err(e) => Err(e),
      }
    }
    Err(e) => Err(e),
  }
}

#[cfg(test)]
mod tests {
  use super::destructure;
  use crate::ast::{Expression, Pattern};
  use crate::runtime::{Value, Verse};
  use std::rc::Rc;
  #[test]
  fn destructure_succeeds() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let _r = destructure(
      v.clone(),
      c.clone(),
      &Pattern::Var(String::from("x"), Box::new(Pattern::Any)),
      &Expression::Int(7),
    );

    let v = c.get_var(String::from("x").to_string());
    assert_eq!(v, Some(Value::Int(7)));
  }

  #[test]
  fn destructure_succeeds_through_array() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let _r = destructure(
      v.clone(),
      c.clone(),
      &Pattern::Array(Some(Box::new(Pattern::Var(
        String::from("x"),
        Box::new(Pattern::Any),
      )))),
      &Expression::Array(vec![Box::new(Expression::Int(7))]),
    );

    let v = c.get_var(String::from("x").to_string());
    assert_eq!(v, Some(Value::Int(7)));
  }
}
