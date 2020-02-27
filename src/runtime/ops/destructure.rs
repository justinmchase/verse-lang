use std::rc::Rc;
use super::super::{
  Scope,
  Value,
  Context,
  exec,
  transform,
  RuntimeError,
  RuntimeError::{
    PatternNotMatchedError
  }
};
use super::super::super::ast::{
  Expression,
  Pattern,
};

pub fn destructure(context: Rc<Context>, pattern: &Pattern, expression: &Expression) -> Result<Value, RuntimeError> {
  match exec(context.clone(), &expression) {
    Ok(value) => {
      let args = Rc::new(vec![value]);
      let scope = Scope::from(args, context);
      match transform(scope, pattern) {
        Ok(m) => {
          if m.matched {
            Ok(m.value)
          } else {
            Err(PatternNotMatchedError)
          }
        },
        Err(e) => Err(e)
      }
    },
    Err(e) => Err(e)
  }
}

#[test]
fn destructure_succeeds() {
  let c = Rc::new(Context::new());
  let _r = destructure(
    c.clone(),
    &Pattern::Var(String::from("x"), Box::new(Pattern::Any)),
    &Expression::Int(7)
  );

  let v = c.get_var(String::from("x").to_string());
  assert_eq!(v, Some(Value::Int(7)));
}

#[test]
fn destructure_succeeds_through_array() {
  let c = Rc::new(Context::new());
  let _r = destructure(
    c.clone(),
    &Pattern::Array(Some(Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))))),
    &Expression::Array(vec![Box::new(Expression::Int(7))])
  );

  let v = c.get_var(String::from("x").to_string());
  assert_eq!(v, Some(Value::Int(7)));
}
