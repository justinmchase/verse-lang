use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::{
  Value
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Context {
  pub outter: Option<Rc<Context>>,
  pub vars: RefCell<HashMap<String, Value>>
}

impl Context {
  pub fn new() -> Self {
    Context {
      outter: None,
      vars: RefCell::new(HashMap::new()),
    }
  }
  
  pub fn from(outter: Rc<Context>) -> Self {
    Context {
      outter: Some(outter.clone()),
      vars: RefCell::new(HashMap::new())
    }
  }
  
  pub fn add_var(&self, name: String, value: Value) {
    self.vars.borrow_mut().insert(name, value);
  }

  pub fn get_var(&self, name: String) -> Option<Value> {
    match self.vars.borrow_mut().get(&name) {
      Some(v) => Some(v.clone()),
      None => match &self.outter {
        Some(o) => o.get_var(name),
        None => None
      }
    }
  }
}
