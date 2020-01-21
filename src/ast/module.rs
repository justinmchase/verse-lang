use std::collections::HashMap;
use super::{
  Expression
};
use super::super::runtime::{
  Value,
  exec,
  Scope,
  RuntimeError
};

pub struct Module {
  pub body: Expression
}


impl Module {
  pub fn new(body: Expression) -> Self {
    Module {
      body
    }
  }

  pub fn export(&mut self) -> Result<Value, RuntimeError> {
    let vars = HashMap::new();
    let mut scope = Scope::new(vec![], vars);
    exec(&mut scope, &self.body)
  }
}