use super::super::runtime::value::Value;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Pattern {
  Default,
  Value(Value),
  Range(Value, Value),
  Quantity(Box<Pattern>, usize, Option<usize>),
  Or(Vec<Box<Pattern>>)
}
