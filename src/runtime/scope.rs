use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::{
  Value
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scope {
  pub input: Rc<Vec<Value>>,
  pub index: Option<usize>,
  pub value: Value,
  pub vars: Rc<RefCell<HashMap<String, Value>>>
}

impl Scope {

  // New top level scope, no vars
  pub fn new(input: Rc<Vec<Value>>) -> Self {
    Scope {
      index: None,
      value: Value::None,
      input,
      vars: Rc::new(RefCell::new(HashMap::new())),
    }
  }

  pub fn empty() -> Self {
    Scope {
      index: None,
      value: Value::None,
      input: Rc::new(vec![]),
      vars: Rc::new(RefCell::new(HashMap::new())),
    }
  }

  pub fn next(&self) -> Option<Scope> {
    let index = self.next_pos();
    match self.input.get(index) {
      Some(value) => {
        Some(Scope {
          index: Some(index),
          value: value.clone(),
          input: self.input.clone(),
          vars: self.vars.clone(),
        })
      },
      None => None
    }
  }

  fn next_pos(&self) -> usize {
    match self.index {
      Some(i) => i + 1,
      None => 0
    }
  }

  pub fn add_var(&self, name: String, value: Value) -> Scope {
    (*self.vars).borrow_mut().insert(name, value);
    Scope {
      index: self.index,
      value: self.value.clone(),
      input: self.input.clone(),
      vars: self.vars.clone()
    }
  }

  pub fn get_var(&self, name: String) -> Option<Value> {
    match (*self.vars).borrow_mut().get(&name) {
      Some(v) => Some(v.clone()),
      None => None
    }
  }

  pub fn clone_vars(&self) -> HashMap<String, Value> {
    (*self.vars).borrow_mut().clone()
  }

  pub fn with_vars(&self, vars: Rc<RefCell<HashMap<String, Value>>>) -> Scope {
    Scope {
      index: self.index,
      value: self.value.clone(),
      input: self.input.clone(),
      vars
    }
  }
}
