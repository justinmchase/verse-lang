#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RuntimeError {
  TransformError,
  NotImplementedError,
  InvalidReferenceError,
  InvalidValueError,
}
