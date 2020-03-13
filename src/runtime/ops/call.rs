use std::rc::Rc;
use super::super::{
  Scope,
  Value,
  Context,
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

fn get_arg(ctx: Rc<Context>, arg: &Option<Box<Expression>>) -> Result<Vec<Value>, RuntimeError> {
  match arg {
    Some(e) => match exec(ctx, e) {
      Ok(v) => Ok(vec![v]),
      Err(e) => Err(e)
    },
    None => Ok(vec![])
  }
}

pub fn call(context: Rc<Context>, exp: &Expression, arg: &Option<Box<Expression>>) -> Result<Value, RuntimeError> {
  match exec(context.clone(), &exp) {
    Ok(value) => {
      match value {
        Value::Function(f, ctx) => {
          match get_arg(context.clone(), arg) {
            Ok(a) => {
              let s = Scope::from(Rc::new(a), ctx);
              let p = Box::new(f.pattern);
              let e = Box::new(f.expression);
              let pattern = Pattern::Project(p, e);
              match transform(s, &pattern) {
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
  let c = Rc::new(Context::new());
  let f = Expression::None;
  let r = call(c, &f, &None);
  assert_eq!(r, Err(NotCallableError(Value::None)));
}

#[test]
fn call_can_call_function() {
  let c = Rc::new(Context::new());
  let f = Expression::Function(
    Box::new(Pattern::Default),
    Box::new(Some(Expression::Int(1)))
  );
  let a = None;
  let r = call(c, &f, &a);
  assert_eq!(r, Ok(Value::Int(1)));
}

#[test]
fn call_expr_can_ref_vars() {
  let c = Rc::new(Context::new());
  c.add_var(String::from("x").to_string(), Value::Int(11));

  let f = Expression::Function(
    Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
    Box::new(Some(Expression::Add(
      Box::new(Expression::Ref(String::from("x"))),
      Box::new(Expression::Ref(String::from("y"))),
    )))
  );

  let a = Box::new(Expression::Int(7));
  let r = call(c, &f, &Some(a));
  assert_eq!(r, Ok(Value::Int(18)));
}
