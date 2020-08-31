use std::fmt;
use std::hash::{ Hash, Hasher };
use data_encoding::HEXLOWER;
use rand::prelude::*;
use super::error::RuntimeError;
use super::value::Value;

const SIZE: usize = 8;

#[derive(Ord, PartialOrd, Eq, Copy, Clone)]
pub struct Id {
  v: [u8; SIZE]
}

impl Id {
  pub fn new() -> Self {
    // todo: accept seed from runtime, incrememt part of the bytes, rand the other
    let mut v = [0u8; SIZE];
    rand::thread_rng().fill_bytes(&mut v);
    Self { v }
  }
  pub fn from(id: &str) -> Result<Id, RuntimeError> {
    match HEXLOWER.decode(id.as_bytes()) {
      Ok(v) => {
        if v.len() != 8 {
          return Err(RuntimeError::InvalidValueError(Value::String(id.to_string())));
        }
        let mut data: [u8; 8] = [0; 8];
        data.copy_from_slice(&v);
        Ok(Id { v: data })
      },
      Err(_) => Err(RuntimeError::InvalidValueError(Value::String(id.to_string())))
    }
  }
	pub fn to_string(&self) -> String {
    HEXLOWER.encode(&self.v)
  }
}

impl PartialEq for Id {
  fn eq(&self, other: &Self) -> bool {
    for x in 0..other.v.len() {
      if self.v[x] != other.v[x] {
        return false;
      }
    }
    return true;
  }
}

impl Default for Id {
  fn default() -> Self {
    Id {
      v: [0; SIZE]
    }
  }
}

impl Into<String> for Id {
  fn into(self) -> String {
    self.to_string()
  }
}

impl Hash for Id {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.v.hash(state);
  }
}

impl fmt::Display for Id {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.to_string())
  }
}

impl fmt::Debug for Id {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:?}", self.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::Id;

  #[test]
  fn id_new() {
    let id = Id::new();
    assert_eq!(16, id.to_string().len());
  }

  #[test]
  fn id_clone_eq() {
    let id0 = Id::new();
    let id1 = id0.clone();
    assert_eq!(id0, id1);
  }

  #[test]
  fn id_neq() {
    let id0 = Id::new();
    let id1 = Id::new();
    assert_ne!(id0, id1);
  }

  #[test]
  fn id_from_string() {
    let id0 = Id::from("23fd985729a5cb83").unwrap();
    assert_eq!(id0.to_string(), String::from("23fd985729a5cb83"));
  }

  #[test]
  fn id_from_same_string_eq() {
    let id0 = Id::from("274ea3d47f8ba4a8").unwrap();
    let id1 = Id::from("274ea3d47f8ba4a8").unwrap();
    assert_eq!(id0, id1);
  }
}
