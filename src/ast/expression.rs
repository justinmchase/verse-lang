#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Expression {
  // literals
  // Int(i32),
  
  // Symbol(String),
  // Tuple(vec![])

  // Unary expressions
  Ref(String),

  // Binary expressions
  // Set(String, Box<Expression>),
  Add(Box<Expression>,Box<Expression>),
  Sub(Box<Expression>,Box<Expression>),
}
