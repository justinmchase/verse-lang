use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use super::{
  Value,
  Match,
  Function,
  Context,
};

#[derive(PartialEq, Eq, Clone)]
pub struct Scope {
  pub origin: Option<Box<Scope>>,
  pub input: Rc<Vec<Value>>,
  pub index: Option<usize>,
  pub value: Value,
  pub stack: Rc<RefCell<VecDeque<Function>>>,
  pub memos: Rc<RefCell<HashMap<(String, Function), Match>>>,
  pub context: Rc<Context>,
}

impl Scope {

  // New top level scope, no vars
  pub fn new(input: Rc<Vec<Value>>) -> Self {
    Scope {
      origin: None,
      index: None,
      value: Value::None,
      input,
      stack: Rc::new(RefCell::new(VecDeque::new())),
      memos: Rc::new(RefCell::new(HashMap::new())),
      context: Rc::new(Context::new()),
    }
  }

  pub fn from(input: Rc<Vec<Value>>, context: Rc<Context>) -> Self {
    Scope {
      origin: None,
      index: None,
      value: Value::None,
      input,
      stack: Rc::new(RefCell::new(VecDeque::new())),
      memos: Rc::new(RefCell::new(HashMap::new())),
      context,
    }
  }

  pub fn empty() -> Self {
    Scope {
      origin: None,
      index: None,
      value: Value::None,
      input: Rc::new(vec![]),
      stack: Rc::new(RefCell::new(VecDeque::new())),
      memos: Rc::new(RefCell::new(HashMap::new())),
      context: Rc::new(Context::new()),
    }
  }

  pub fn with(&self, context: Rc<Context>) -> Self {
    Scope {
      origin: self.origin.clone(),
      index: self.index.clone(),
      value: self.value.clone(),
      input: self.input.clone(),
      stack: self.stack.clone(),
      memos: self.memos.clone(),
      context
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
          stack: self.stack.clone(),
          memos: self.memos.clone(),
          context: self.context.clone()
        })
      },
      None => None
    }
  }

  pub fn step_into(&self) -> Option<Scope> {
    match &self.value {
      Value::Array(items) => Some(Scope {
        origin: Some(Box::new(self.clone())),
        index: None,
        value: Value::None,
        input: Rc::new(items.to_vec()),
        stack: self.stack.clone(),
        memos: self.memos.clone(),
        context: self.context.clone(),
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

  pub fn position(&self) -> String {
    match self.index {
      Some(i) => match &self.origin {
        Some(o) => format!("{}/{}", o, i),
        None => format!("/{}", i),
      },
      None => String::from("/"),
    }
  }

  pub fn push_stack(&self, func: &Function) {
    (*self.stack).borrow_mut().push_back(func.clone())
  }

  pub fn peek_stack(&self) -> Option<Function> {
    match (*self.stack).borrow_mut().back() {
      Some(r) => Some(r.clone()),
      None => None
    }
  }

  pub fn pop_stack(&self) {
    (*self.stack).borrow_mut().pop_back();
  }
  
  pub fn set_memo(&self, f: &Function, m: &Match) {
    (*self.memos).borrow_mut().insert((self.position(), f.clone()), m.clone());
  }
  
  pub fn get_memo(&self, f: &Function) -> Option<Match> {
    match (*self.memos).borrow_mut().get(&(self.position(), f.clone())) {
      Some(m) => Some(m.clone()),
      None => match &self.origin {
        Some(o) => o.get_memo(f),
        None => None
      }
    }
  }
}

impl fmt::Display for Scope {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.position())
  }
}

impl fmt::Debug for Scope {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Scope {{ position: {:?}, value: {:?}, stack: {:?}, memos: {} }}",
      self.position(),
      self.value,
      (*self.stack).borrow(),
      (*self.memos).borrow().len()
   )
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