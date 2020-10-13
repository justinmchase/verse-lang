use crate::runtime::{Library, Name, Resolver, RuntimeError};
use semver::Version;
use std::collections::HashMap;
use std::rc::Rc;

use crate::runtime::modules::unicode;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MemoryResolver {
  libraries: HashMap<(Name, Version), Rc<Library>>,
}

impl MemoryResolver {
  pub fn new(libs: Vec<Rc<Library>>) -> Self {
    let mut libraries = HashMap::new();
    for lib in libs {
      let name = lib.name();
      let version = lib.version();
      libraries.insert((name.clone(), version.clone()), lib);
    }
    MemoryResolver { libraries }
  }

  pub fn core() -> Result<Self, RuntimeError> {
    let mut libraries = HashMap::new();
    libraries = unicode(libraries)?;
    Ok(MemoryResolver { libraries })
  }
}

impl Resolver for MemoryResolver {
  fn resolve(&self, name: &Name, version: &Version) -> Result<Rc<Library>, RuntimeError> {
    // for lib in self.libraries

    println!(
      "MemoryResolver.Resolve: ({:?},{:?}) in {:?}",
      name, version, self.libraries
    );
    let lib = self.libraries.get(&(name.clone(), version.clone()));
    println!("MemoryResolver.Resolved: {:?}", lib);
    match lib {
      Some(lib) => Ok(lib.clone()),
      None => Err(RuntimeError::NotResolvedError),
    }
  }
}
