#[derive(Debug)]
pub enum Expression {
  // literals
  Int(i32),

  // Unary expressions
  Ret(Option<Box<Expression>>),
  Arg(usize),
  Ref(String),

  // Binary expressions
  Set((String, Box<Expression>)),
  Add((Box<Expression>,Box<Expression>)),
  Sub((Box<Expression>,Box<Expression>)),
}
