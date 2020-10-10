use crate::ast::FieldExpression;
use crate::ast::ImportExpression;
use crate::ast::Pattern;
use crate::runtime::Value;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
  // Misc
  None,

  // Literals
  Int(i32),
  String(String),
  Pattern(Box<Pattern>),
  Array(Vec<Box<Expression>>),
  Object(Vec<FieldExpression>),
  Value(Value),

  // Unary expressions
  Ref(String),
  Call(Box<Expression>, Option<Box<Expression>>),
  Destructure(Box<Pattern>, Box<Expression>), // [x,y,z] <- [1,2,3]
  Import(ImportExpression),

  // Binary expressions
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),

  // N-ary expressions
  Block(Vec<Box<Expression>>),
}
