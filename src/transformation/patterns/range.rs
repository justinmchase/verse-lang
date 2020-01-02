use super::super::scope::Scope;
use super::super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::super::super::runtime::value::{
  Value,
  value_ge,
  value_le
};

pub fn range(scope: &mut Scope, left: Value, right: Value) -> Result<Value, TransformError> {
  match scope.peek() {
    None => Err(Fail),
    Some(v) => {
      if value_ge(v, &left) && value_le(v, &right) {
        let ret = Ok(v.clone());
        scope.next();
        return ret;
      } else {
        return Err(Fail);
      }
    }
  }
}

#[test]
fn eos() {
  let mut scope = Scope::new(vec![]);
  let res = range(&mut scope, Value::Int(0), Value::Int(1));
  assert_eq!(res, Err(Fail));
}

#[test]
fn int_min() {
  let mut scope = Scope::new(vec![Value::Int(0)]);
  let res = range(&mut scope, Value::Int(0), Value::Int(2));
  assert_eq!(res, Ok(Value::Int(0)));
}

#[test]
fn int_mid() {
  let mut scope = Scope::new(vec![Value::Int(1)]);
  let res = range(&mut scope, Value::Int(0), Value::Int(2));
  assert_eq!(res, Ok(Value::Int(1)));
}

#[test]
fn int_max() {
  let mut scope = Scope::new(vec![Value::Int(2)]);
  let res = range(&mut scope, Value::Int(0), Value::Int(2));
  assert_eq!(res, Ok(Value::Int(2)));
}

#[test]
fn int_fail() {
  let mut scope = Scope::new(vec![Value::Int(3)]);
  let res = range(&mut scope, Value::Int(0), Value::Int(2));
  assert_eq!(res, Err(Fail));
}
