extern crate semver;
use crate::ast::{Expression, ImportExpression};
use crate::runtime::{exec, Context, Id, Library, Name, Namespace, Resolver, RuntimeError, Value};
use semver::Version;
// use std::cell::RefCell;
// use std::collections::HashMap;
use std::rc::Rc;

pub struct Verse {
  id: Id,
  main: Rc<Library>,
  resolvers: Vec<Box<dyn Resolver>>,
  // cache: Rc<RefCell<HashMap<(Name, Version, Namespace), Value>>>,
}

impl PartialEq for Verse {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}
impl Eq for Verse {}

impl Verse {
  pub fn default() -> Self {
    let main = Rc::new(Library::default());
    Verse {
      id: Id::new(),
      main: main.clone(),
      resolvers: vec![],
    }
  }
  pub fn new(main: Rc<Library>, resolvers: Vec<Box<dyn Resolver>>) -> Self {
    Verse {
      id: Id::new(),
      main: main.clone(),
      resolvers,
    }
  }

  // fn get_args(arg: Option<Value>) -> Vec<Value> {
  //   match arg {
  //     Some(v) => vec![v],
  //     None => vec![]
  //   }
  // }

  pub fn get_main(&self) -> Rc<Library> {
    self.main.clone()
  }
  pub fn create_context(&self) -> Rc<Context> {
    Rc::new(Context::new(self.main.clone()))
  }

  pub fn run(verse: Rc<Verse>, arg: Option<Value>) -> Result<Value, RuntimeError> {
    let arg_exp = match arg {
      Some(v) => Some(Box::new(Expression::Value(v))),
      None => None,
    };

    let exp = Expression::Call(
      Box::new(Expression::Import(ImportExpression::Main)),
      arg_exp,
    );

    let context = Rc::new(Context::new(verse.main.clone()));
    exec(verse, context, &exp)
  }

  pub fn resolve_main(verse: Rc<Verse>) -> Result<Value, RuntimeError> {
    Verse::resolve_module(verse.clone(), verse.main.clone(), None)
  }

  pub fn resolve_library(
    verse: Rc<Verse>,
    name: &Name,
    version: &Version,
    namespace: Option<&Namespace>,
  ) -> Result<Value, RuntimeError> {
    println!("resolve_library {:?} {:?}", name, version);
    for resolver in &verse.resolvers {
      println!("resolver {:?} ", resolver);
      match resolver.resolve(name, version) {
        Ok(lib) => return Verse::resolve_module(verse.clone(), lib, namespace),
        Err(e) => match e {
          RuntimeError::NotResolvedError => continue,
          _ => return Err(e),
        },
      }
    }

    // Unable to resolve a library with the current resolvers
    println!("resolve_library unable to resolve...");
    Err(RuntimeError::NotResolvedError)
  }

  pub fn resolve_module(
    verse: Rc<Verse>,
    library: Rc<Library>,
    namespace: Option<&Namespace>,
  ) -> Result<Value, RuntimeError> {
    // todo: cache the results of module resolve

    println!("resolve_module {:?}", namespace);
    let context = Rc::new(Context::new(library.clone()));
    library.resolve_module(verse, context, namespace)

    // Err(RuntimeError::NotImplemented)
  }
}
