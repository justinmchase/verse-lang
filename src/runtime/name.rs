use regex::Regex;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Name(String);

impl Name {
  pub fn parse(name: &str) -> Self {
    lazy_static! {
      static ref NAME_SCHEMA: Regex = Regex::new(r"^\w(\w|[0-9])+$").unwrap();
    }
    assert!(NAME_SCHEMA.is_match(name));
    Name(name.into())
  }
}

impl From<&str> for Name {
  fn from(name: &str) -> Self {
    Name::parse(name)
  }
}

impl From<&Name> for String {
  fn from(name: &Name) -> Self {
    name.0.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::Name;
  #[test]
  fn can_make_valid_name() {
    Name::parse("test");
  }

  #[test]
  #[should_panic]
  fn cant_make_an_invalid_name() {
    Name::parse("bad name");
  }
}
