use super::super::runtime::{
  Value
};
use super::Pattern;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
  // literals
  Literal(Value),

  // Unary expressions
  Ref(&'static str),
  Call(Box<Expression>, Vec<Box<Expression>>),
  Destructure(Box<Pattern>, Box<Expression>), // [x,y,z] = [1,2,3]
  Return(Box<Expression>),

  // Binary expressions
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),

  // N-ary expressions
  Block(Vec<Box<Expression>>),
}
