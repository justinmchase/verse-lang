use super::super::super::ast::{
  Pattern
};
use super::super::{
  Scope,
  Match,
  RuntimeError,
};
use super::{
  and,
  any,
  array,
  project,
  then,
  var,
  default,
  equal,
  or,
  quantity,
  r#type,
};

pub fn transform(scope: Scope, pattern: &Pattern) -> Result<Match, RuntimeError> {
  println!();
  println!("var: {:?}", scope.clone());
  println!("pat: {:?}", pattern.clone());
  match pattern {
    Pattern::And(p) => and(scope, p),
    Pattern::Any => any(scope),
    Pattern::Array(p) => array(scope, p),
    Pattern::Default => default(scope),
    Pattern::Equal(v) => equal(scope, v),
    Pattern::Or(p) => or(scope, p),
    Pattern::Project(p, expr) => project(scope, p, expr),
    Pattern::Quantity(p, min, max) => quantity(scope, p, min, max),
    Pattern::Then(p) => then(scope, p),
    Pattern::Type(t) => r#type(scope, t),
    Pattern::Var(name, p) => var(scope, name.to_string(), p),
  }
}