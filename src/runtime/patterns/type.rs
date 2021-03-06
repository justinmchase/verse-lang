use crate::ast::Pattern;
use crate::runtime::{Match, RuntimeError, Scope, Type, Value};
use std::rc::Rc;

pub fn r#type(start: Scope, t: &Type) -> Result<Match, RuntimeError> {
  let n = start.next();
  match n.clone() {
    Some(end) => {
      let value = &end.value;
      match value {
        Value::None => {
          if t == &Type::None {
            return Ok(Match::ok(value.clone(), start, end));
          }
        }
        Value::Int(_i) => {
          if t == &Type::Int {
            return Ok(Match::ok(value.clone(), start, end));
          }
        }
        Value::String(_s) => {
          if t == &Type::String {
            return Ok(Match::ok(value.clone(), start, end));
          }
        }
        Value::Array(_a) => {
          if t == &Type::Array {
            return Ok(Match::ok(value.clone(), start, end));
          }
        }
        Value::Pattern(_p, _c) => {
          if t == &Type::Pattern {
            return Ok(Match::ok(value.clone(), start, end));
          }
        }
        Value::Object(_id, _o) => {
          if t == &Type::Object {
            return Ok(Match::ok(value.clone(), start, end));
          }
        }
      }
    }
    None => (),
  }
  println!("Type Match Fail: {:?} is not {:?}", n, t);
  Ok(Match::fail(start))
}

#[cfg(test)]
mod tests {
  use super::r#type;
  use super::Type;
  use crate::ast::Pattern;
  use crate::runtime::{Context, Scope, Value, Verse};
  use std::rc::Rc;

  #[test]
  fn type_matches_correct_values() {
    let v = Verse::default();
    let values = vec![
      (Value::None, Type::None),
      (Value::Int(0), Type::Int),
      (Value::String("".to_string()), Type::String),
      (Value::Array(vec![]), Type::Array),
      (
        Value::Pattern(Box::new(Pattern::Default), Rc::new(Context::default())),
        Type::Pattern,
      ),
    ];

    for (v, t) in values.iter() {
      let verse = Verse::default();
      let s = Scope::new(Rc::new(verse), Rc::new(vec![v.clone()]));
      let m0 = r#type(s.clone(), &Type::None).unwrap();
      let m1 = r#type(s.clone(), &Type::Int).unwrap();
      let m2 = r#type(s.clone(), &Type::String).unwrap();
      let m3 = r#type(s.clone(), &Type::Array).unwrap();
      let m4 = r#type(s.clone(), &Type::Pattern).unwrap();

      assert_eq!(m0.matched, *t == Type::None);
      assert_eq!(m1.matched, *t == Type::Int);
      assert_eq!(m2.matched, *t == Type::String);
      assert_eq!(m3.matched, *t == Type::Array);
      assert_eq!(m4.matched, *t == Type::Pattern);

      assert_eq!(m0.value, Value::None);
      assert_eq!(
        m1.value,
        if *t == Type::Int {
          v.clone()
        } else {
          Value::None
        }
      );
      assert_eq!(
        m2.value,
        if *t == Type::String {
          v.clone()
        } else {
          Value::None
        }
      );
      assert_eq!(
        m3.value,
        if *t == Type::Array {
          v.clone()
        } else {
          Value::None
        }
      );
      assert_eq!(
        m4.value,
        if *t == Type::Pattern {
          v.clone()
        } else {
          Value::None
        }
      );
    }
  }
}
