use crate::ast::Pattern;
use crate::runtime::{Context, RuntimeError, Value, Verse};
use std::rc::Rc;

pub fn pattern(_: Rc<Verse>, c: Rc<Context>, p: &Pattern) -> Result<Value, RuntimeError> {
  Ok(Value::Pattern(Box::new(p.clone()), c.clone()))
}

#[cfg(test)]
mod tests {
  use super::pattern;
  use crate::ast::Pattern;
  use crate::runtime::{Value, Verse};
  use std::rc::Rc;

  #[test]
  fn pattern_returns_pattern() {
    let v = Verse::default();
    let c = v.create_context();
    let r = pattern(Rc::new(v), c.clone(), &Pattern::Default);
    assert_eq!(r, Ok(Value::Pattern(Box::new(Pattern::Default), c.clone())));
  }
}
