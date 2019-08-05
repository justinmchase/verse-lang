use super::ast::module::{Exportable, Module};
use super::ast::function::Function;
use super::runtime::value::Value;
use super::runtime::scope::Scope;

pub struct Verse {
  pub root: Box<Module>
}

impl Verse {
  pub fn new(root: Module) -> Self {
    Verse {
      root: Box::new(root)
    }
  }

  fn invokefn(&self, f: &Box<Function>, arguments: Vec<Value>) -> Value {
    println!("invokefn: {}", f.name);

    let mut scope = Scope::new(arguments);
    for p in f.parameters.iter() {
      scope.exec(p)
    }
    for e in f.body.iter() {
      scope.exec(e);
    }
    scope.ret
  }

  pub fn invoke(&self, name: &str, arguments: Vec<Value>) -> Value {
    println!("invoke: {}", name);
    let exported = self.root.exports
      .iter()
      .find(|&ex| match ex {
        Exportable::Function(f) if f.name == name => true,
        _ => false
      });

    match exported {
      Some(Exportable::Function(f)) => self.invokefn(f, arguments),
      None => panic!("Unknown function {}", name)
    }
  }
}