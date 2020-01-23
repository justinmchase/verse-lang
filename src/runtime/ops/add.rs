use super::super::{
  Scope,
  Value,
  exec,
  RuntimeError,
  RuntimeError::{
    InvalidValueError
  }
};
use super::super::super::ast::Expression;

pub fn add(scope: Scope, left: &Expression, right: &Expression) -> Result<Value, RuntimeError> {
  let r0 = exec(scope.clone(), &left);
  if r0.is_err() { return r0; }
  
  let r1 = exec(scope, &right);
  if r1.is_err() { return r1; }

  let l = r0.ok().unwrap();
  let r = r1.ok().unwrap();
  match l {
    Value::Int(li32) => match r {
      Value::Int(ri32) => Ok(Value::Int(li32 + ri32)),
      _ => Err(InvalidValueError)
    },
    Value::String(lstr) => match r {
      Value::String(rstr) => Ok(Value::String(format!("{}{}", lstr, rstr))),
      _ => Err(InvalidValueError)
    },
    _ => Err(InvalidValueError)
  }
}
