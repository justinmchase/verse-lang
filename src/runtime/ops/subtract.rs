use std::rc::Rc;
use super::super::super::ast::Expression;
use super::super::{
  Scope,
  Value,
  exec,
  RuntimeError,
  RuntimeError::{
    InvalidValueError
  }
};

pub fn subtract(scope: Scope, left: &Expression, right: &Expression) -> Result<Value, RuntimeError> {
  let r0 = exec(scope.clone(), left);
  if r0.is_err() { return r0; }
  
  let r1 = exec(scope.clone(), right);
  if r1.is_err() { return r1; }

  let l = r0.ok().unwrap();
  let r = r1.ok().unwrap();
  match l {
    Value::Int(li32) => match r {
      Value::Int(ri32) => Ok(Value::Int(li32 - ri32)),
      _ => Err(InvalidValueError)
    },
    _ => Err(InvalidValueError)
  }
}

#[test]
fn sub_tests() {
  let values = vec![
    (Value::Int(3), Value::Int(2), Ok(Value::Int(1))),
    (Value::String("a".to_string()), Value::String("b".to_string()), Err(InvalidValueError)),
    (Value::String("a".to_string()), Value::Int(0), Err(InvalidValueError)),
    (Value::Int(0), Value::String("a".to_string()), Err(InvalidValueError)),
    (Value::None, Value::None, Err(InvalidValueError)),

    // tood: all combinations...
  ];
  for (l, r, v) in values.iter() {
    let left = Expression::Literal(l.clone());
    let right = Expression::Literal(r.clone());
    let s = Scope::new(Rc::new(vec![]));
    let res = subtract(s, &left, &right);
    assert_eq!(res, *v);
  }
}