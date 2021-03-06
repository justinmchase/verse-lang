use crate::ast::Pattern;
use crate::runtime::{Context, Match, Value, Verse};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

#[derive(Eq, Clone)]
pub struct Scope {
  pub origin: Option<Box<Scope>>,
  pub input: Rc<Vec<Value>>,
  pub index: Option<usize>,
  pub value: Value,
  pub stack: Rc<RefCell<VecDeque<Pattern>>>,
  pub memos: Rc<RefCell<HashMap<(String, Pattern), Match>>>,
  pub verse: Rc<Verse>,
  pub context: Rc<Context>,
}

impl Scope {
  pub fn default() -> Self {
    let verse = Rc::new(Verse::default());
    Scope::empty(verse)
  }
  pub fn empty(verse: Rc<Verse>) -> Self {
    Scope {
      origin: None,
      index: None,
      value: Value::None,
      input: Rc::new(vec![]),
      stack: Rc::new(RefCell::new(VecDeque::new())),
      memos: Rc::new(RefCell::new(HashMap::new())),
      verse: verse.clone(),
      context: verse.create_context(),
    }
  }

  pub fn new(verse: Rc<Verse>, input: Rc<Vec<Value>>) -> Self {
    Scope {
      origin: None,
      index: None,
      value: Value::None,
      input,
      stack: Rc::new(RefCell::new(VecDeque::new())),
      memos: Rc::new(RefCell::new(HashMap::new())),
      verse: verse.clone(),
      context: verse.create_context(),
    }
  }

  pub fn from(verse: Rc<Verse>, context: Rc<Context>, input: Rc<Vec<Value>>) -> Self {
    Scope {
      origin: None,
      index: None,
      value: Value::None,
      input,
      stack: Rc::new(RefCell::new(VecDeque::new())),
      memos: Rc::new(RefCell::new(HashMap::new())),
      verse: verse.clone(),
      context: context.clone(),
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
      verse: self.verse.clone(),
      context,
    }
  }

  pub fn next(&self) -> Option<Scope> {
    let index = self.next_pos();
    match self.input.get(index) {
      Some(value) => Some(Scope {
        origin: self.origin.clone(),
        index: Some(index),
        value: value.clone(),
        input: self.input.clone(),
        stack: self.stack.clone(),
        memos: self.memos.clone(),
        verse: self.verse.clone(),
        context: self.context.clone(),
      }),
      None => None,
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
        verse: self.verse.clone(),
        context: self.context.clone(),
      }),
      _ => None,
    }
  }

  fn next_pos(&self) -> usize {
    match self.index {
      Some(i) => i + 1,
      None => 0,
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

  pub fn push_stack(&self, pattern: &Pattern) {
    (*self.stack).borrow_mut().push_back(pattern.clone())
  }

  pub fn peek_stack(&self) -> Option<Pattern> {
    match (*self.stack).borrow_mut().back() {
      Some(r) => Some(r.clone()),
      None => None,
    }
  }

  pub fn pop_stack(&self) {
    (*self.stack).borrow_mut().pop_back();
  }
  pub fn set_memo(&self, p: &Pattern, m: &Match) {
    (*self.memos)
      .borrow_mut()
      .insert((self.position(), p.clone()), m.clone());
  }
  pub fn get_memo(&self, p: &Pattern) -> Option<Match> {
    // Careful here, a `.borrow_mut()` causes a panic due to already being borrowed
    // This can essentially be called in a recursive manner and its important to
    // keep the borrow mut on RefCell's to only mutations
    match self.memos.borrow().get(&(self.position(), p.clone())) {
      Some(m) => Some(m.clone()),
      None => match &self.origin {
        Some(o) => o.get_memo(p),
        None => None,
      },
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

impl PartialOrd for Scope {
  fn partial_cmp(&self, other: &Scope) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Scope {
  fn eq(&self, other: &Scope) -> bool {
    self.origin == other.origin && self.index == other.index
  }
}

impl Ord for Scope {
  fn cmp(&self, other: &Scope) -> Ordering {
    if self.origin.is_some() && other.origin.is_some() {
      match self.origin.cmp(&other.origin) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => self.index.cmp(&other.index),
      }
    } else if self.origin.is_none() && other.origin.is_none() {
      self.index.cmp(&other.index)
    } else if self.origin.is_some() && other.origin.is_none() {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::runtime::{Scope, Value, Verse};
  use std::rc::Rc;
  #[test]
  fn position_at_empty() {
    let v = Verse::default();
    let s = Scope::empty(Rc::new(v));
    assert_eq!("/", s.position());
  }

  #[test]
  fn position_at_first_index() {
    let v = Verse::default();
    let s0 = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(0)]));
    let s1 = s0.next().unwrap();
    assert_eq!("/0", s1.position());
  }

  #[test]
  fn postition_into_array() {
    let v = Verse::default();
    let s0 = Scope::new(Rc::new(v), Rc::new(vec![Value::Array(vec![Value::Int(0)])]));
    let s1 = s0.next().unwrap(); // array at 0
    let s2 = s1.step_into().unwrap(); // into array at 0
    let s3 = s2.next().unwrap(); // int at index 0 of array
    assert_eq!("/0/0", s3.position());
  }

  #[test]
  fn scope_two_empty_scopes_are_equal() {
    let v = Rc::new(Verse::default());
    let l = Scope::empty(v.clone());
    let r = Scope::empty(v.clone());
    assert_eq!(l, r);
  }

  #[test]
  fn scope_next_next_greater_than() {
    let v = Verse::default();
    let s = Scope::new(Rc::new(v), Rc::new(vec![Value::Int(1), Value::Int(2)]));
    let l = s.next().unwrap();
    let r = l.next().unwrap();
    assert_ne!(l, r);
    assert_ne!(l, s);
    assert_ne!(r, s);
    assert!(l < r);
    assert!(r > l);
    assert!(l > s);
    assert!(r > s);
  }

  #[test]
  fn scope_with_different_origins_are_not_equal() {
    let v = Verse::default();
    let s = Scope::new(
      Rc::new(v),
      Rc::new(vec![
        Value::Array(vec![Value::Int(1)]),
        Value::Array(vec![Value::Int(1)]),
      ]),
    );

    // They should both have the same value and index but from different origins
    let l = s.next().unwrap().step_into().unwrap().next().unwrap();
    let r = s
      .next()
      .unwrap()
      .next()
      .unwrap()
      .step_into()
      .unwrap()
      .next()
      .unwrap();

    assert_ne!(l, r);
    assert_ne!(l, s);
    assert_ne!(r, s);
    assert!(l < r);
    assert!(r > l);
    assert!(l > s);
    assert!(r > s);
  }
}
