use std::rc::Rc;
use std::cell::RefCell;
use super::super::{
  Scope,
  Value,
  exec,
  transform,
  RuntimeError,
  RuntimeError::{
    NotCallableError,
    PatternNotMatchedError
  }
};
use super::super::super::ast::{
  Expression,
  Pattern,
};

fn get_arg(s: Scope, arg: &Option<Box<Expression>>) -> Result<Vec<Value>, RuntimeError> {
  match arg {
    Some(e) => match exec(s, e) {
      Ok(v) => Ok(vec![v]),
      Err(e) => Err(e)
    },
    None => Ok(vec![])
  }
}

pub fn call(start: Scope, func: &Expression, arg: &Option<Box<Expression>>) -> Result<Value, RuntimeError> {
  match exec(start.clone(), &func) {
    Ok(value) => {
      match value {
        Value::Function(p, e, v) => {
          match get_arg(start, arg) {
            Ok(a) => {
              let vars = Rc::new(RefCell::new(v));
              let scope = Scope::new(Rc::new(a)).with_vars(vars);
              match transform(scope, &Pattern::Project(p, e)) {
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
        },
        _ => Err(NotCallableError(value)) // cannot call a non-function
      }
    },
    Err(e) => Err(e)
  }
}

#[test]
fn call_cannot_call_non_function() {
  let s = Scope::new(Rc::new(vec![]));
  let f = Expression::Literal(Value::None);
  let r = call(s, &f, &None);
  assert_eq!(r, Err(NotCallableError(Value::None)));
}

#[test]
fn call_can_call_function() {
  let s = Scope::new(Rc::new(vec![]));
  let f = Expression::Function(
    Box::new(Pattern::Default),
    Box::new(Expression::Literal(Value::Int(1)))
  );
  let a = None;
  let r = call(s, &f, &a);
  assert_eq!(r, Ok(Value::Int(1)));
}

#[test]
fn call_expr_can_ref_vars() {
  let s = Scope::new(Rc::new(vec![]));
  s.add_var("x".to_string(), Value::Int(11));

  let f = Expression::Function(
    Box::new(Pattern::Var("y", Box::new(Pattern::Any))),
    Box::new(Expression::Add(
      Box::new(Expression::Ref("x")),
      Box::new(Expression::Ref("y")),
    )
  ));

  let a = Box::new(Expression::Literal(Value::Int(7)));
  let r = call(s, &f, &Some(a));
  assert_eq!(r, Ok(Value::Int(18)));
}
