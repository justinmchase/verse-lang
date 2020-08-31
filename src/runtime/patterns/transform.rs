use crate::ast::{
  Pattern
};
use crate::runtime::{
  Scope,
  Match,
  RuntimeError,
};
use crate::runtime::patterns::{
  and,
  any,
  array,
  project,
  project_native,
  then,
  var,
  default,
  equal,
  or,
  quantity,
  r#ref,
  r#type,
};

pub fn transform(scope: Scope, pattern: &Pattern) -> Result<Match, RuntimeError> {
  println!("pattern: {:?}", pattern.clone());
  println!("  scope: {}", scope.clone());
  match pattern {
    Pattern::And(p) => and(scope, p),
    Pattern::Any => any(scope),
    Pattern::Array(p) => array(scope, p),
    Pattern::Default => default(scope),
    Pattern::Equal(v) => equal(scope, v),
    Pattern::Or(p) => or(scope, p),
    Pattern::Project(p, expr) => project(scope, p, expr),
    Pattern::ProjectNative(p, h) => project_native(scope, p, h),
    Pattern::Quantity(p, min, max) => quantity(scope, p, min, max),
    Pattern::Ref(name) => r#ref(scope, name.to_string()),
    Pattern::Then(p) => then(scope, p),
    Pattern::Type(t) => r#type(scope, t),
    Pattern::Var(name, p) => var(scope, name.to_string(), p),
  }
}