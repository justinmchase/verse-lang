use crate::runtime::{Library, Name, RuntimeError};
use semver::Version;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Resolver: Debug {
  fn resolve(&self, name: &Name, version: &Version) -> Result<Rc<Library>, RuntimeError>;
}
