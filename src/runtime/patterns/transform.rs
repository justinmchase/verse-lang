use super::super::super::ast::{
  Pattern
};
use super::super::{
  Value,
  Scope,
  RuntimeError,
};
use super::{
  and,
  any,
  array,
  project,
  var,
};

pub fn transform(scope: &mut Scope, pattern: &Pattern) -> Result<Value, RuntimeError> {
  println!("mat: {:?} {:?}", pattern, scope.peek());
  match pattern {
    Pattern::And(p) => and(scope, p),
    Pattern::Any => any(scope),
    Pattern::Array(p) => array(scope, p),
    Pattern::Project(p, expr) => project(scope, p, expr),
    Pattern::Var(name, p) => var(scope, name.to_string(), p),
    _ => Err(RuntimeError::NotImplementedError)
  }
}