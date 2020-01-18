use super::expression::Expression;
use super::pattern::Pattern;
use super::pattern::Pattern::{Project};

#[derive(Hash)]
pub struct Function {
  pub name: String,
  pub body: Pattern
}

impl Function {
  pub fn new(name: &'static str, pattern: Pattern, expression: Expression) -> Self {
    Function {
      name: name.to_string(),
      body: Project(
        Box::new(pattern),
        Box::new(expression)
      )
    }
  }
}
