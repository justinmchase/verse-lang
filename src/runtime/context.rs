use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::{
  Value
};

#[derive(PartialEq, Eq, Clone)]
pub struct Context {
  outter: Option<Rc<Context>>,
  vars: RefCell<HashMap<String, Value>>
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
    self.vars.borrow_mut().insert(name.to_string(), value);
  }

  pub fn get_var(&self, name: String) -> Option<Value> {
    match self.vars.borrow().get(&name) {
      Some(v) => Some(v.clone()),
      None => match &self.outter {
        Some(o) => o.get_var(name),
        None => None
      }
    }
  }
}

impl fmt::Display for Context {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // let vars = self.vars.clone();
    write!(f,
      "Context {{ outter: {:?}, vars: {:?} }}",
      self.outter,
      self.vars.clone().into_inner().keys()
    )
  }
}

impl fmt::Debug for Context {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Context {{  }}"
   )
  }
}


