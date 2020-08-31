use crate::ast::Expression;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FieldExpression {
  Ref(String),
  Set(String, Box<Expression>),
  Spread(Box<Expression>)
}
