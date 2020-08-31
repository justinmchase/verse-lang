use crate::runtime::{
  Value,
  Type,
  Namespace
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RuntimeError {
  TransformError,
  NotImplementedError,
  ScopeEmptyError,
  InvalidReferenceError(String),
  InvalidValueError(Value),
  InvalidTypeError(Value, Type), // (Actual, Expected)
  InvalidNameError(String),
  InvalidNamespaceError(Namespace),
  NotCallableError(Value),
  DuplicateNamespaceError(String),
  PatternNotMatchedError,
  IndirectLeftRecursion,
  NotResolvedError
}
