use super::pattern::Pattern;

#[derive(Debug)]
pub enum Expression {
  // literals
  Int(i32),
  
  // Symbol(String),
  // Tuple(vec![])
  // Pattern(Box<Pattern>)

  // Unary expressions
  Ret(Option<Box<Expression>>),
  Arg(usize),
  Ref(String),

  // Binary expressions
  Set((String, Box<Expression>)),
  Add((Box<Expression>,Box<Expression>)),
  Sub((Box<Expression>,Box<Expression>)),
}
