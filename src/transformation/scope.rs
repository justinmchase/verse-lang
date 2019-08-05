use super::super::runtime::value::Value;

pub struct Scope {
  index: usize,
  input: Vec<Value>
}

impl Scope {
  pub fn new(input: Vec<Value>) -> Self {
    Scope {
      index: 0,
      input
    }
  }

  pub fn peek(&self) -> Option<&Value> {
    self.input.get(self.index)
  }

  pub fn next(&mut self) {
    self.index = self.index + 1;
  }
}
