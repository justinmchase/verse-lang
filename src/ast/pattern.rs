use super::expression::Expression;
use super::super::runtime::Value;

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
  Var(&'static str, Box<Pattern>),
}
