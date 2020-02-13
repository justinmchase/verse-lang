use super::super::runtime::{
  Value
};
use super::Pattern;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
  // literals
  None,
  Int(i32),
  String(String),
  Function(Box<Pattern>, Box<Expression>),
  Array(Vec<Box<Expression>>),

  // Unary expressions
  Ref(&'static str),
  Call(Box<Expression>, Option<Box<Expression>>),
  Destructure(Box<Pattern>, Box<Expression>), // [x,y,z] = [1,2,3]

  // Binary expressions
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),

  // N-ary expressions
  Block(Vec<Box<Expression>>),
}
