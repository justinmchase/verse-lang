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
  var,
  default,
};

pub fn transform(scope: Scope, pattern: &Pattern) -> Result<Match, RuntimeError> {
  println!("  p: {:?}", pattern.clone());
  println!("var: {:?}", scope.clone());
  match pattern {
    Pattern::And(p) => and(scope, p),
    Pattern::Any => any(scope),
    Pattern::Array(p) => array(scope, p),
    Pattern::Project(p, expr) => project(scope, p, expr),
    Pattern::Var(name, p) => var(scope, name.to_string(), p),
    Pattern::Default => default(scope),
  }
}