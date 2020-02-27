use std::rc::Rc;
use super::super::{
  Value,
  Context,
  exec,
  RuntimeError,
  RuntimeError::{
    InvalidValueError
  }
};
use super::super::super::ast::Expression;

pub fn add(context: Rc<Context>, left: &Expression, right: &Expression) -> Result<Value, RuntimeError> {
  let r0 = exec(context.clone(), &left);
  if r0.is_err() { return r0; }
  
  let r1 = exec(context.clone(), &right);
  if r1.is_err() { return r1; }

  let l = r0.ok().unwrap();
  let r = r1.ok().unwrap();
  match l {
    Value::Int(li32) => match r {
      Value::Int(ri32) => Ok(Value::Int(li32 + ri32)),
      _ => Err(InvalidValueError(r))
    },
    Value::String(lstr) => match r {
      Value::String(rstr) => Ok(Value::String(format!("{}{}", lstr, rstr))),
      _ => Err(InvalidValueError(r))
    },
    _ => Err(InvalidValueError(l))
  }
}

#[test]
fn add_tests() {
  let values = vec![
    (Expression::Int(1), Expression::Int(2), Ok(Value::Int(3))),
    (Expression::String("a".to_string()), Expression::String("b".to_string()), Ok(Value::String("ab".to_string()))),
    (Expression::String("a".to_string()), Expression::Int(0), Err(InvalidValueError(Value::Int(0)))),
    (Expression::Int(0), Expression::String("a".to_string()), Err(InvalidValueError(Value::String("a".to_string())))),
    (Expression::None, Expression::None, Err(InvalidValueError(Value::None))),

    // tood: all combinations...
  ];
  for (l, r, v) in values.iter() {
    let c = Rc::new(Context::new());
    let res = add(c, l, r);
    assert_eq!(res, *v);
  }
}
