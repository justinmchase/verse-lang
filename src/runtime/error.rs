use super::Value;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RuntimeError {
  TransformError,
  NotImplementedError,
  ScopeEmptyError,
  InvalidArgumentError,
  InvalidReferenceError(String),
  InvalidValueError(Value),
  NotCallableError(Value),
  PatternNotMatchedError,
  IndirectLeftRecursion
}
