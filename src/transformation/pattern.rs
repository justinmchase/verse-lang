use super::super::runtime::{Value};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Pattern {
  Any,
  Default,
  Value(Value),
  Range(Value, Value),
  Quantity(Box<Pattern>, usize, Option<usize>),
  Or(Vec<Box<Pattern>>)
}
