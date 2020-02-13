use std::rc::Rc;
use std::collections::HashMap;
use super::super::super::ast::{
  Pattern,
  Expression,
};
use super::super::{
  Scope,
  Match,
  Value,
  Type,
  RuntimeError
};

pub fn r#type(start: Scope, t: &Type) -> Result<Match, RuntimeError> {
  let n = start.next();
  match n {
    Some(end) => {
      let value = &end.value;
      match value {
        Value::None => if t == &Type::None {
          return Ok(Match::ok(value.clone(), start, end));
        },
        Value::Int(i) => if t == &Type::Int {
          return Ok(Match::ok(value.clone(), start, end));
        },
        Value::String(s) => if t == &Type::String {
          return Ok(Match::ok(value.clone(), start, end));
        },
        Value::Array(a) => if t == &Type::Array {
          return Ok(Match::ok(value.clone(), start, end));
        },
        Value::Function(p, e, v) => if t == &Type::Function {
          return Ok(Match::ok(value.clone(), start, end));
        }
      }
    },
    None => ()
  }
  Ok(Match::fail(start))
}

#[test]
fn type_matches_correct_values() {
  let values = vec![
    (Value::None, Type::None),
    (Value::Int(0), Type::Int),
    (Value::String("".to_string()), Type::String),
    (Value::Array(vec![]), Type::Array),
    (Value::Function(Box::new(Pattern::Any), Box::new(Expression::Literal(Value::None)), HashMap::new()), Type::Function),
  ];

  for (v, t) in values.iter() {
    let s = Scope::new(Rc::new(vec![v.clone()]));
    let m0 = r#type(s.clone(), &Type::None).unwrap();
    let m1 = r#type(s.clone(), &Type::Int).unwrap();
    let m2 = r#type(s.clone(), &Type::String).unwrap();
    let m3 = r#type(s.clone(), &Type::Array).unwrap();
    let m4 = r#type(s.clone(), &Type::Function).unwrap();

    assert_eq!(m0.matched, *t == Type::None);
    assert_eq!(m1.matched, *t == Type::Int);
    assert_eq!(m2.matched, *t == Type::String);
    assert_eq!(m3.matched, *t == Type::Array);
    assert_eq!(m4.matched, *t == Type::Function);

    assert_eq!(m0.value, Value::None);
    assert_eq!(m1.value, if *t == Type::Int { v.clone() } else { Value::None });
    assert_eq!(m2.value, if *t == Type::String { v.clone() } else { Value::None });
    assert_eq!(m3.value, if *t == Type::Array { v.clone() } else { Value::None });
    assert_eq!(m4.value, if *t == Type::Function { v.clone() } else { Value::None });
  }
}