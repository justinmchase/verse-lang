use crate::ast::Pattern;
use crate::ast::FieldExpression;
use crate::ast::ImportExpression;
use crate::runtime::Value;
use crate::runtime::NativeFunctionHandler;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
  // Misc
  None,

  // Literals
  Int(i32),
  String(String),
  Function(Box<Pattern>, Box<Option<Expression>>),
  NativeFunction(Box<Pattern>, NativeFunctionHandler),
  Array(Vec<Box<Expression>>),
  Object(Vec<FieldExpression>),
  Value(Value),

  // Unary expressions
  Ref(String),
  Call(Box<Expression>, Option<Box<Expression>>),
  Destructure(Box<Pattern>, Box<Expression>), // [x,y,z] = [1,2,3]
  Import(ImportExpression),

  // Binary expressions
  Add(Box<Expression>, Box<Expression>),
  Sub(Box<Expression>, Box<Expression>),

  // N-ary expressions
  Block(Vec<Box<Expression>>),
}
