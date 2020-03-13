use super::Pattern;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FieldExpression {
  Ref(String),
  Set(String, Box<Expression>),
  Spread(Box<Expression>)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
  // literals
  None,
  Int(i32),
  String(String),
  Function(Box<Pattern>, Box<Option<Expression>>),
  Array(Vec<Box<Expression>>),
  Object(Vec<FieldExpression>),

  // Unary expressions
  Ref(String),
  Call(Box<Expression>, Option<Box<Expression>>),
  Destructure(Box<Pattern>, Box<Expression>), // [x,y,z] = [1,2,3]

  // Binary expressions
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),

  // N-ary expressions
  Block(Vec<Box<Expression>>),
}
