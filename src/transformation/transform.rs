use super::scope::Scope;
use super::pattern::Pattern;
use super::super::runtime::value::Value;
use super::error::{
  TransformError
};
use super::patterns::{
  or,
  quantity,
  range,
  value,
};

pub fn transform(scope: &mut Scope, pattern: Pattern) -> Result<Value, TransformError> {
  match pattern {
    Pattern::Default => Ok(Value::None),
    Pattern::Value(v) => value(scope, v),
    Pattern::Range(v0, v1) => range(scope, v0, v1),
    Pattern::Quantity(p, min, max) => quantity(scope, *p, min, max),
    Pattern::Or(patterns) => or(scope, patterns)
  }
}
