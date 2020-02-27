use std::rc::Rc;
use super::super::ast::{
  Module,
  Pattern,
};
use super::{
  Context,
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
    let global = Rc::new(Context::new());
    let ex = self.root.export(global);
    match ex {
      Ok(v) => match v {
        Value::Function(f, ctx) => {
          let args = Rc::new(Verse::get_args(arg));
          let scope = Scope::from(args, ctx);
          println!();
          println!("----");
          println!("run: {}", scope.clone());
          let p = f.pattern;
          let e = f.expression;
          match transform(scope, &Pattern::Project(Box::new(p), Box::new(e))) {
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