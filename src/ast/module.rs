use std::rc::Rc;
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
    let scope = Scope::new(Rc::new(vec![]));
    exec(scope, &self.body)
  }
}