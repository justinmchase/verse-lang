use super::super::ast::{
  Module,
  Function
};
use super::{
  Value,
  Scope,
  RuntimeError,
  RuntimeError::{
    InvalidReferenceError
  },
  transform
};

pub struct Verse {
  pub root: Box<Module>
}

impl Verse {
  pub fn new(root: Module) -> Self {
    Verse {
      root: Box::new(root)
    }
  }

  fn invokefn(&self, f: &Function, arg: Value) -> Result<Value, RuntimeError> {
    println!("invokefn: {}", f.name);
    let mut scope = Scope::new(vec![arg]);
    transform(&mut scope, &f.body)
  }

  pub fn invoke(&self, name: &str, arg: Value) -> Result<Value, RuntimeError> {
    println!("invoke: {}", name);
    let exported = self.root.exports
      .iter()
      .find(|&ex| ex.name == name);

    match exported {
      Some(f) => self.invokefn(f, arg),
      None => Err(InvalidReferenceError)
    }
  }
}