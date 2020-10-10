use crate::ast::{Expression, Pattern};
use crate::runtime::{exec, transform, Context, RuntimeError, Scope, Value, Verse};
use std::rc::Rc;

fn get_arg(
  verse: Rc<Verse>,
  context: Rc<Context>,
  arg: &Option<Box<Expression>>,
) -> Result<Vec<Value>, RuntimeError> {
  match arg {
    Some(e) => match exec(verse, context, e) {
      Ok(v) => Ok(vec![v]),
      Err(e) => Err(e),
    },
    None => Ok(vec![]),
  }
}

pub fn call(
  verse: Rc<Verse>,
  context: Rc<Context>,
  exp: &Expression,
  arg: &Option<Box<Expression>>,
) -> Result<Value, RuntimeError> {
  match exec(verse.clone(), context.clone(), &exp) {
    Ok(value) => {
      match value.clone() {
        Value::Pattern(p, ctx) => match get_arg(verse.clone(), context.clone(), arg) {
          Ok(a) => {
            let s = Scope::from(verse, ctx, Rc::new(a));
            match transform(s, &p) {
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
        },
        _ => Err(RuntimeError::NotCallableError(value)), // cannot call a non-function
      }
    }
    Err(e) => Err(e),
  }
}

#[cfg(test)]
mod tests {
  use super::call;
  use crate::ast::{Expression, Pattern};
  use crate::runtime::{RuntimeError, Value, Verse};
  use std::rc::Rc;
  #[test]
  fn call_cannot_call_non_function() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let f = Expression::None;
    let r = call(v, c, &f, &None);
    assert_eq!(r, Err(RuntimeError::NotCallableError(Value::None)));
  }

  #[test]
  fn call_can_call_function() {
    // let v = Rc::new(Verse::default());
    // let c = v.create_context();
    // let f = Expression::Function(
    //   Box::new(Pattern::Default),
    //   Box::new(Some(Expression::Int(1))),
    //   None,
    // );
    // let a = None;
    // let r = call(v, c, &f, &a);
    // assert_eq!(r, Ok(Value::Int(1)));
  }

  #[test]
  fn call_expr_can_ref_vars() {
    // let v = Rc::new(Verse::default());
    // let c = v.create_context();
    // c.add_var(String::from("x").to_string(), Value::Int(11));

    // let f = Expression::Function(
    //   Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
    //   Box::new(Some(Expression::Add(
    //     Box::new(Expression::Ref(String::from("x"))),
    //     Box::new(Expression::Ref(String::from("y"))),
    //   ))),
    //   None,
    // );

    // let a = Box::new(Expression::Int(7));
    // let r = call(v, c, &f, &Some(a));
    // assert_eq!(r, Ok(Value::Int(18)));
  }
}
