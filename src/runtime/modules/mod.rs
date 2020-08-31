use crate::runtime::{
  Library,
  RuntimeError
};

pub use sys::sys;
mod sys;

pub fn global() -> Result<Vec<Library>, RuntimeError> {
  Ok(vec![ sys()? ])
}
