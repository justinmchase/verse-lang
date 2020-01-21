use std::collections::HashMap;
use super::{
  Value
};

#[derive(Debug)]
pub struct Scope {
  index: usize,
  input: Vec<Value>,
  pub vars: HashMap<String, Value>
}

impl Scope {

  // New top level scope, no vars
  pub fn new(input: Vec<Value>, vars: HashMap<String, Value>) -> Self {
    Scope {
      index: 0,
      input,
      vars,
    }
  }
  
  pub fn peek(&mut self) -> Option<&Value> {
    self.input.get(self.index)
  }

  pub fn next(&mut self) -> Option<&Value> {
    match self.input.get(self.index) {
      Some(v) => {
        self.index = self.index + 1;
        Some(v)
      },
      None => None
    }
  }

  pub fn pos(&mut self) -> usize {
    self.index
  }

  pub fn mov(&mut self, index: usize) {
    self.index = index;
  }

  

  // pub fn get(&mut self, name: String) -> Option<&Value> {
  //   self.vars.get(&name)
  // }

  // pub fn set(&mut self, name: String, value: Value) {
    
  //   println!("set: {:?} = {:?}", name, value.clone());
  //   self.vars.insert(name, value);
  //   println!("vars: {:?}", self.vars.clone());
  // }
}
