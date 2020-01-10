use super::super::scope::Scope;
use super::super::error::{
  TransformError,
  TransformError::{Fail}
};
use super::super::super::runtime::{Value};

pub fn value(scope: &mut Scope, v: Value) -> Result<Value, TransformError> {
  match scope.peek() {
    None => Err(Fail),
    Some(val) => match val.clone() {
      Value::None => match v {
        Value::None => Ok(Value::None),
        _ => Err(Fail)
      },
      Value::Int(i) => match v {
        Value::Int(j) if i == j => {
          scope.next();
          Ok(v)
        },
        _ => Err(Fail)
      },
      Value::String(a) => match v {
        Value::String(b) => {
          if a == b {
            scope.next();
            Ok(Value::String(a))
          } else {
            Err(Fail)
          }
        },
        _ => Err(Fail)
      },
      Value::Array(_a) => Err(Fail)
    }
  }
}
