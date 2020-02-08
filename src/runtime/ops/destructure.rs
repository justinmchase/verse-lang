use std::rc::Rc;
use super::super::{
  Scope,
  Value,
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

pub fn destructure(start: Scope, pattern: &Pattern, expression: &Expression) -> Result<Value, RuntimeError> {
  match exec(start.clone(), &expression) {
    Ok(value) => {
      let args = Rc::new(vec![value]);
      let scope = Scope::new(args).with_vars(start.vars);
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
  let s = Scope::empty();
  let r = destructure(
    s.clone(),
    &Pattern::Var("x", Box::new(Pattern::Any)),
    &Expression::Literal(Value::Int(7))
  );

  let v = s.get_var("x".to_string());
  assert_eq!(v, Some(Value::Int(7)));
}

#[test]
fn destructure_succeeds_through_array() {
  let s = Scope::empty();
  let r = destructure(
    s.clone(),
    &Pattern::Array(Some(Box::new(Pattern::Var("x", Box::new(Pattern::Any))))),
    &Expression::Literal(Value::Array(vec![Value::Int(7)]))
  );

  let v = s.get_var("x".to_string());
  assert_eq!(v, Some(Value::Int(7)));
}
