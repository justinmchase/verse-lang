use super::expression::Expression;
use super::super::runtime::Value;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Pattern {
  And(Vec<Box<Pattern>>),
  Any,
  Array(Option<Box<Pattern>>),
  Default,
  Equal(Value),
  Project(Box<Pattern>, Box<Expression>),
  Then(Vec<Box<Pattern>>),
  Var(&'static str, Box<Pattern>),
}
