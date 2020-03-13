use super::Value;
use super::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RuntimeError {
  TransformError,
  // NotImplementedError,
  ScopeEmptyError,
  InvalidReferenceError(String),
  InvalidValueError(Value),
  InvalidTypeError(Value, Type), // (Actual, Expected)
  NotCallableError(Value),
  PatternNotMatchedError,
  IndirectLeftRecursion
}
