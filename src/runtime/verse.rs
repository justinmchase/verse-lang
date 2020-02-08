use std::rc::Rc;
use std::cell::RefCell;
use super::super::ast::{
  Module,
  Pattern,
};
use super::{
  Value,
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

  fn get_args(arg: Option<Value>) -> Vec<Value> {
    match arg {
      Some(v) => vec![v],
      None => vec![]
    }
  }

  pub fn run(&mut self, arg: Option<Value>) -> Result<Value, RuntimeError> {
    let ex = self.root.export();
    match ex {
      Ok(v) => match v {
        Value::Function(p, e, v) => {
          let args = Rc::new(Verse::get_args(arg));
          let vars = Rc::new(RefCell::new(v));
          let scope = Scope::new(args).with_vars(vars);
          println!();
          println!("----");
          println!("run: {:?}", scope.clone());
          match transform(scope, &Pattern::Project(p, e)) {
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