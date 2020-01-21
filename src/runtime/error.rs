use super::Value;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RuntimeError {
  TransformError,
  NotImplementedError,
  InvalidReferenceError(String),
  InvalidValueError,
  NotCallableError(Value)
}
