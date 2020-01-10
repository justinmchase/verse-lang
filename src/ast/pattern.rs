use super::expression::Expression;
use super::super::runtime::{ Value };

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Pattern {
  Any,
  Var(&'static str, Box<Pattern>),
  Tuple(Vec<Box<Pattern>>)
  // Default,
  // Value(Value),
  // Range(Value, Value),
  // Quantity(Box<Pattern>, usize, Option<usize>),
  // Or(Vec<Box<Pattern>>)
}
