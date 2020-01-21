use std::collections::HashMap;
use super::super::ast::{
  Module,
  Pattern,
  Pattern::{Project}
};
use super::{
  Value,
  Value::{
    Function
  },
  Scope,
  RuntimeError,
  RuntimeError::{
    InvalidReferenceError,
    NotCallableError
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
          let vars = HashMap::new();
          let mut scope = Scope::new(vec![arg], vars);
          transform(&mut scope, &Project(
            pat,
            exp
          ))
        },
        _ => Err(NotCallableError(v))
      },
      Err(e) => Err(e)
    }
  }
}