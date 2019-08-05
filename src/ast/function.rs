use super::expression::Expression;

pub struct Function {
  pub name: &'static str,
  pub parameters: Vec<Expression>,
  pub body: Vec<Expression>
}

