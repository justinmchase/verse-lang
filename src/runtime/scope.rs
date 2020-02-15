use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::{
  Value
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scope {
  pub origin: String,
  pub input: Rc<Vec<Value>>,
  pub index: Option<usize>,
  pub value: Value,
  pub vars: Rc<RefCell<HashMap<String, Value>>>
}

impl Scope {

  // New top level scope, no vars
  pub fn new(input: Rc<Vec<Value>>) -> Self {
    Scope {
      origin: String::from("/"),
      index: None,
      value: Value::None,
      input,
      vars: Rc::new(RefCell::new(HashMap::new())),
    }
  }

  pub fn empty() -> Self {
    Scope {
      origin: String::from("/"),
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
          origin: self.origin.clone(),
          index: Some(index),
          value: value.clone(),
          input: self.input.clone(),
          vars: self.vars.clone(),
        })
      },
      None => None
    }
  }

  pub fn step_into(&self) -> Option<Scope> {
    match &self.value {
      Value::Array(items) => Some(Scope {
        origin: format!("{}{}/", self.origin, self.index.unwrap()),
        index: None,
        value: Value::None,
        input: Rc::new(items.to_vec()),
        vars: self.vars.clone()
      }),
      _ => None
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
      origin: self.origin.clone(),
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
      origin: self.origin.clone(),
      index: self.index,
      value: self.value.clone(),
      input: self.input.clone(),
      vars
    }
  }

  pub fn with_origin(&self, origin: String) -> Scope {
    Scope {
      origin,
      index: self.index,
      value: self.value.clone(),
      input: self.input.clone(),
      vars: self.vars.clone()
    }
  }

  pub fn position(&self) -> String {
    match self.index {
      Some(i) => format!("{}{}", self.origin, i),
      None => self.origin.clone(),
    }
  }
}

#[test]
fn position_at_empty() {
  let s = Scope::empty();
  assert_eq!("/", s.position());
}

#[test]
fn position_at_first_index() {
  let s0 = Scope::new(Rc::new(vec![Value::Int(0)]));
  let s1 = s0.next().unwrap();
  assert_eq!("/0", s1.position());
}

#[test]
fn postition_into_array() {
  let s0 = Scope::new(Rc::new(vec![Value::Array(vec![Value::Int(0)])]));
  let s1 = s0.next().unwrap(); // array at 0
  let s2 = s1.step_into().unwrap(); // into array at 0
  let s3 = s2.next().unwrap(); // int at index 0 of array
  assert_eq!("/0/0", s3.position());
}