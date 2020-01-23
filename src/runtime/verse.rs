use std::rc::Rc;
use super::super::ast::{
  Module,
  Pattern::{
    Project
  }
};
use super::{
  Value,
  Value::{
    Function
  },
  Scope,
  RuntimeError,
  RuntimeError::{
    NotCallableError,
    PatternNotMatchedError,
  },
  transform
};

pub struct Verse {
  pub root: Module
}

impl Verse {
  pub fn new(root: Module) -> Self {
    Verse {
      root
    }
  }

  pub fn run(&mut self, arg: Value) -> Result<Value, RuntimeError> {
    let ex = self.root.export();
    match ex {
      Ok(v) => match v {
        Function(pat, exp) => {
          let args = Rc::new(vec![arg]);
          let scope = Scope::new(args);
          match transform(scope, &Project(pat, exp)) {
            Ok(m) => {
              if m.matched {
                Ok(m.value)
              } else {
                Err(PatternNotMatchedError)
              }
            },
            Err(e) => Err(e)
          }
        },
        _ => Err(NotCallableError(v))
      },
      Err(e) => Err(e)
    }
  }
}