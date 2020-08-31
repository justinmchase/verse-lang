extern crate semver;
use semver::Version;
use crate::runtime::{
  Namespace,
  Name,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ImportExpression {
  // A reference to an external module
  Global(Name, Version, Option<Namespace>),

  // A module relative to the root of this library
  Library(Option<Namespace>),

  // A ref to the main module of the current verse
  Main
}
