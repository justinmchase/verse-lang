use super::super::super::ast::{
  Pattern
};
use super::super::{
  Value,
  Scope,
  transform,
  RuntimeError,
  RuntimeError::{
    TransformError
  }
};

pub fn array(scope: &mut Scope, pattern: &Pattern) -> Result<Value, RuntimeError> {
  match scope.peek() {
    Some(v) => match v {
      Value::Array(items) => {
        let mut s = Scope::from(items.to_vec(), scope);
        let res = transform(&mut s, pattern);
        if res.is_ok() {
          scope.next();
          for (k, v) in s.vars.iter() {
            scope.vars.insert(k.to_string(), v.clone());
          }
        }
        println!("  s: {:?}", scope.vars);
        return res;
      },
      _ => Err(TransformError) // todo: implement tuple
    }
    None => Err(TransformError)
  }
}