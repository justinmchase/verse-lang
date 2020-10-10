use crate::runtime::{Id, Library, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone)]
pub struct Context {
  id: Id,
  library: Rc<Library>,
  outter: Option<Rc<Context>>,
  vars: RefCell<HashMap<String, Value>>,
}

impl Context {
  pub fn default() -> Self {
    Context {
      id: Id::new(),
      library: Rc::new(Library::default()),
      outter: None,
      vars: RefCell::new(HashMap::new()),
    }
  }

  pub fn new(library: Rc<Library>) -> Self {
    Context {
      id: Id::new(),
      library: library,
      outter: None,
      vars: RefCell::new(HashMap::new()),
    }
  }
  pub fn from(outter: Rc<Context>) -> Self {
    Context {
      id: Id::new(),
      library: outter.library.clone(),
      outter: Some(outter.clone()),
      vars: RefCell::new(HashMap::new()),
    }
  }

  pub fn get_library(&self) -> Rc<Library> {
    self.library.clone()
  }
  pub fn add_var(&self, name: String, value: Value) {
    self.vars.borrow_mut().insert(name.to_string(), value);
  }

  pub fn get_var(&self, name: String) -> Option<Value> {
    match self.vars.borrow().get(&name) {
      Some(v) => Some(v.clone()),
      None => match &self.outter {
        Some(o) => o.get_var(name),
        None => None,
      },
    }
  }
}

impl Hash for Context {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

impl fmt::Display for Context {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // let vars = self.vars.clone();
    write!(f, "Context({})", self.id)
  }
}

impl fmt::Debug for Context {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Context({}, {:?})", self.id, self.vars)
  }
}
