use super::super::scope::Scope;
use super::super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::super::super::runtime::value::Value;

pub fn range(scope: &mut Scope, v0: Value, v1: Value) -> Result<Value, TransformError> {
  println!("rng {:?}", scope.peek());
  match scope.peek() {
    None => Err(Fail),
    Some(val) => match val.clone() {
      Value::None => Err(Fail),
      Value::Int(i) => match v0 {
        Value::Int(j) if j < i => match v1 {
          Value::Int(k) if i < k => {
            scope.next();
            Ok(Value::Int(i))
          },
          _ => Err(Fail)
        },
        _ => Err(Fail)
      },
      Value::String(i) => match v0 {
        Value::String(j) => {
          if j < i {
            match v1 {
              Value::String(k) => {
                if i < k {
                  scope.next();
                  return Ok(Value::String(i));
                } else {
                  return Err(Fail);
                }
              },
              _ => Err(Fail)
            }
          } else {
            return Err(Fail);
          }
        },
        _ => Err(Fail)
      },
      _ => Err(Fail)
    }
  }
}
