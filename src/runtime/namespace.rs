extern crate regex;
use std::hash::Hash;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Namespace(String);

impl Namespace {
  pub fn parse(namespace: &str) -> Self {
    lazy_static! {
      static ref NAMESPACE_SCHEMA: Regex = Regex::new(r"^(\w(\w|[0-9])+)(\.\w(\w|[0-9])+)*$").unwrap();
    }
    assert!(NAMESPACE_SCHEMA.is_match(namespace));
    Namespace(namespace.into())
  }
}

impl From<&str> for Namespace {
  fn from(namespace: &str) -> Self {
    Namespace::parse(namespace)
  }
}

impl From<&Namespace> for String {
  fn from(namespace: &Namespace) -> Self {
    namespace.0.clone()
  }
}


#[cfg(test)]
mod tests {
  use super::Namespace;

  #[test]
  fn can_make_valid_namespace() {
    Namespace::parse("test");
  }

  #[test]
  fn can_make_valid_nested_namespace() {
    Namespace::parse("test.foo.bar");
  }

  #[test]
  #[should_panic]
  fn cant_make_an_invalid_namespace() {
    Namespace::parse("bad.name space.test");
  }
}
