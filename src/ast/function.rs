use super::expression::Expression;
use super::pattern::Pattern;

pub struct Function {
  pub name: &'static str,
  pub pattern: Option<Pattern>,
  pub body: Vec<Expression>
}

