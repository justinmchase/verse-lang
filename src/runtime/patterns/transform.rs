use crate::ast::Pattern;
use crate::runtime::patterns::{
  and, any, array, default, equal, native, or, project, quantity, r#ref, r#type, then, var,
};
use crate::runtime::{Match, RuntimeError, Scope};

pub fn transform(scope: Scope, pattern: &Pattern) -> Result<Match, RuntimeError> {
  println!("pattern: {:?}", pattern.clone());
  println!("  scope: {}", scope.clone());
  match pattern {
    Pattern::And(p) => and(scope, p),
    Pattern::Any => any(scope),
    Pattern::Array(p) => array(scope, p),
    Pattern::Default => default(scope),
    Pattern::Equal(v) => equal(scope, v),
    Pattern::Native(h) => native(scope, *h),
    Pattern::Or(p) => or(scope, p),
    Pattern::Project(p, e) => project(scope, p, e),
    Pattern::Quantity(p, min, max) => quantity(scope, p, min, max),
    Pattern::Ref(name) => r#ref(scope, name.to_string()),
    Pattern::Then(p) => then(scope, p),
    Pattern::Type(t) => r#type(scope, t),
    Pattern::Var(name, p) => var(scope, name.to_string(), p),
  }
}
