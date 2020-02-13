use super::expression::Expression;
use super::super::runtime::{
  Value,
  Type
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern {
  And(Vec<Box<Pattern>>),
  Any,
  Array(Option<Box<Pattern>>),
  Default,
  Equal(Value),
  Or(Vec<Box<Pattern>>),
  Project(Box<Pattern>, Box<Expression>),
  Quantity(Box<Pattern>, Option<usize>, Option<usize>),
  Then(Vec<Box<Pattern>>),
  Type(Type),
  Var(String, Box<Pattern>),
}
