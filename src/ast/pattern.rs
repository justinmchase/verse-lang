use super::expression::Expression;
// use super::super::runtime::{ Value };

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Pattern {
  And(Vec<Box<Pattern>>),
  Any,
  Array(Option<Box<Pattern>>),
  Project(Box<Pattern>, Box<Expression>),
  Var(&'static str, Box<Pattern>),
  // Default,
  // Value(Value),
  // Range(Value, Value),
  // Quantity(Box<Pattern>, usize, Option<usize>),
  // Or(Vec<Box<Pattern>>)
}
