use super::super::ast::{
  Expression,
  Pattern,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Function {
  pub pattern: Pattern,
  pub expression: Option<Expression>,
}

impl Function {
  pub fn new(pattern: &Pattern, expression: &Option<Expression>) -> Self {
    Function {
      pattern: pattern.clone(),
      expression: expression.clone(),
    }
  }

  pub fn default() -> Self {
    Function {
      pattern: Pattern::Default,
      expression: None,
    }
  }
}