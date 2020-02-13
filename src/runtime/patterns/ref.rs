use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::super::super::ast::{
  Pattern,
  Expression
};
use super::super::{
  Scope,
  Match,
  Value,
  RuntimeError,
  transform
};

// e.g.
//
// a = "hi" -> 1
// b = "by" -> 2
// c = a | b
//
// 1 = c"hi"
// 2 = c"by"

pub fn r#ref(start: Scope, name: String) -> Result<Match, RuntimeError> {
  let n = &name;
  let var = start.get_var(n.clone());
  match var {
    Some(v) => match v {
      Value::Function(p, e, v) => {
        let pattern = Pattern::Project(p, e);
        let s = start.with_vars(Rc::new(RefCell::new(v)));
        transform(s, &pattern)
      },
      _ => {
        let pattern = Pattern::Equal(v);
        transform(start, &pattern)
      }
    },
    None => Err(RuntimeError::InvalidReferenceError(n.clone())),
  }
}

#[test]
fn ref_projects_function() {
  // a = "hi" -> 1
  // 1 = a
  let s = Scope::new(Rc::new(vec![Value::String(String::from("hi"))]));
  let f = Value::Function(
    Box::new(Pattern::Equal(Value::String(String::from("hi")))),
    Box::new(Expression::Int(1)),
    HashMap::new()
  );
  s.add_var(String::from("a"), f);
  let m = r#ref(s, String::from("a")).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Int(1));
}

#[test]
fn ref_matches_equal_value() {
  let values = vec![
    Value::None,
    Value::Int(1),
    Value::String(String::from("test")),
    Value::Array(vec![]),
  ];
  for v in values.iter() {
    let s = Scope::new(Rc::new(vec![v.clone()]));
    s.add_var(String::from("x"), v.clone());
    let m = r#ref(s, String::from("x")).unwrap();
    assert_eq!(m.matched, true);
    assert_eq!(m.value, *v);
  }
}
