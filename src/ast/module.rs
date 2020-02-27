use std::rc::Rc;
use super::{
  Expression
};
use super::super::runtime::{
  Value,
  Context,
  exec,
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

  pub fn export(&mut self, global: Rc<Context>) -> Result<Value, RuntimeError> {
    let moduleContext = Rc::new(Context::from(global));
    exec(moduleContext, &self.body)
  }
}