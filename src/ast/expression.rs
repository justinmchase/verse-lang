use super::super::runtime::{
  Value
};
use super::Pattern;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
  // literals
  Literal(Value),
  Function(Box<Pattern>, Box<Expression>),

  // Unary expressions
  Ref(&'static str),
  Call(Box<Expression>, Vec<Box<Expression>>),
  Destructure(Box<Pattern>, Box<Expression>), // [x,y,z] = [1,2,3]

  // Binary expressions
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),

  // N-ary expressions
  Block(Vec<Box<Expression>>),
}
