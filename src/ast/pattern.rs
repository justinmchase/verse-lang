use super::super::runtime::{NativePatternHandler, Type, Value};
use super::expression::Expression;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Pattern {
  // Value
  Any,
  Default,
  Type(Type),
  Equal(Value),
  Array(Option<Box<Pattern>>),
  // todo: Object(fields)

  // Quantifiers
  Quantity(Box<Pattern>, Option<usize>, Option<usize>),
  // Logical
  And(Vec<Box<Pattern>>),
  Or(Vec<Box<Pattern>>),
  Then(Vec<Box<Pattern>>),

  // Meta
  Ref(String),
  Var(String, Box<Pattern>),
  Project(Box<Pattern>, Box<Option<Expression>>),
  Native(NativePatternHandler),
}
