extern crate regex;
use crate::ast::{Expression, Pattern};
use crate::runtime::{exec, Context, Name, Namespace, RuntimeError, Value, Verse};
use semver::Version;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Library {
  name: Name,
  version: Version,
  modules: RefCell<HashMap<Namespace, Expression>>,
}

impl Library {
  pub fn default() -> Self {
    Library {
      name: "default".into(),
      version: Version::parse("0.0.0").unwrap(),
      modules: RefCell::new(HashMap::new()),
    }
  }

  pub fn new(name: &Name, version: &Version) -> Self {
    Library {
      name: name.clone(),
      version: version.clone(),
      modules: RefCell::new(HashMap::new()),
    }
  }

  pub fn name(&self) -> &Name {
    &self.name
  }

  pub fn version(&self) -> &Version {
    &self.version
  }

  pub fn resolve_module(
    &self,
    verse: Rc<Verse>,
    context: Rc<Context>,
    namespace: Option<&Namespace>,
  ) -> Result<Value, RuntimeError> {
    let modules = self.modules.borrow();
    let expression = match namespace {
      Some(n) => match modules.get(n).clone() {
        Some(m) => Ok(m.clone()),
        None => Err(RuntimeError::InvalidNamespaceError(n.clone())),
      },
      None => match modules.get(&"main".into()) {
        Some(m) => Ok(m.clone()),
        None => Err(RuntimeError::InvalidNamespaceError(Namespace::from("main"))),
      },
    };
    println!("  module: {:?}", expression);
    match expression {
      Ok(exp) => match exec(verse, context.clone(), &exp) {
        Ok(res) => match res.clone() {
          Value::Pattern(_, _) => Ok(res),
          _ => Ok(Value::Pattern(
            Box::new(Pattern::Project(
              Box::new(Pattern::Default),
              Box::new(Some(Expression::Value(res))),
            )),
            context.clone(),
          )),
        },
        Err(e) => Err(e),
      },
      Err(e) => Err(e),
    }
  }

  pub fn add_main(&self, module: Expression) -> Result<&Self, RuntimeError> {
    self.add_module(&Namespace::parse("main"), module)
  }

  pub fn add_module(
    &self,
    namespace: &Namespace,
    module: Expression,
  ) -> Result<&Self, RuntimeError> {
    let mut modules = self.modules.borrow_mut();
    match modules.get(&namespace) {
      Some(_) => Err(RuntimeError::DuplicateNamespaceError(namespace.into())),
      None => {
        modules.insert(namespace.clone(), module.clone());
        Ok(self)
      }
    }
  }
}

impl Hash for Library {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
    self.version.hash(state);
  }
}

#[cfg(test)]
mod tests {
  use super::Library;
  use crate::ast::Expression;
  use crate::runtime::{Name, Namespace, RuntimeError};
  use semver::Version;

  #[test]
  fn can_make_valid_library_name() {
    Library::new(&Name::from("test"), &Version::parse("1.0.0").unwrap());
  }

  #[test]
  #[should_panic]
  fn can_not_make_invalid_library_name() {
    let name = "not a valid name";
    Library::new(&Name::from(name), &Version::parse("1.0.0").unwrap());
  }

  #[test]
  fn can_add_main_module() -> Result<(), RuntimeError> {
    let lib = Library::new(&Name::from("test"), &Version::parse("1.0.0").unwrap());
    lib.add_module(&Namespace::parse("main"), Expression::None)?;
    Ok(())
  }

  #[test]
  fn can_add_sub_modules() -> Result<(), RuntimeError> {
    let lib = Library::new(&Name::from("test"), &Version::parse("1.0.0").unwrap());
    lib.add_module(&Namespace::parse("foo.bar"), Expression::None)?;
    lib.add_module(&Namespace::parse("foo.bar.baz"), Expression::None)?;
    Ok(())
  }

  #[test]
  fn can_not_add_duplicate_modules() -> Result<(), RuntimeError> {
    let lib = Library::new(&Name::from("test"), &Version::parse("1.0.0").unwrap());
    let res = lib
      .add_module(&Namespace::parse("foo.bar"), Expression::None)?
      .add_module(&Namespace::parse("foo.bar"), Expression::None);
    assert_eq!(
      res,
      Err(RuntimeError::DuplicateNamespaceError(String::from(
        "foo.bar"
      )))
    );
    Ok(())
  }

  // #[test]
  // fn can_resolve_main_module() -> Result<(), RuntimeError> {
  //   let lib = Rc::new(Library::new(&Name::from("test"), &Version::parse("1.0.0").unwrap()));
  //   let module = Rc::new(Module::default(Rc::downgrade(&lib)));
  //   lib.add_module("main", module)?;

  //   // lib.resolve
  //   Ok(())
  // }
}
