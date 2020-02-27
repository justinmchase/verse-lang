use std::fmt;
use std::rc::Rc;
use std::hash::{ Hash, Hasher };
use super::{
  Function,
  Context,
};

#[derive(PartialEq, Eq, Clone)]
pub enum Value {
  None,
  Int(i32),
  String(String),
  Array(Vec<Value>),
  Function(Box<Function>, Rc<Context>)
}

impl Hash for Value {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Value::None => None::<i32>.hash(state),
      Value::Int(i) => i.hash(state),
      Value::String(s) => s.hash(state),
      Value::Array(v) => v.hash(state),
      Value::Function(f, _) => {
        // omit context to prevent recursion
        &f.hash(state);
      }
    }
  }
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::None => write!(f, "None"),
      Value::Int(i) => write!(f, "Int({:?})", i),
      Value::String(s) => write!(f, "String({:?})", s),
      Value::Array(v) => write!(f, "Array({:?})", v),
      Value::Function(func, _) => write!(f, "Function({:?})", func),
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::None => write!(f, "None"),
      Value::Int(i) => write!(f, "Int({})", i),
      Value::String(s) => write!(f, "String({})", s),
      Value::Array(v) => write!(f, "Array({:?})", v),
      Value::Function(func, _) => write!(f, "Function{:?}", func)
    }
  }
}



pub fn value_cmp(left: &Value, right: &Value) -> Option<i8> {
  if left == right { return Some(0); }
  match left {
    Value::None => match right {
      Value::None => Some(0),
      _ => None
    },
    Value::Int(l) => match right {
      Value::Int(r) => {
        if l == r { return Some(0) }
        if l >  r { return Some(1) }
        if l <  r { return Some(-1) }
        return None
      },
      _ => None
    },
    Value::String(l) => match right {
      Value::String(r) => {
        if l == r { return Some(0) }
        if l >  r { return Some(1) }
        if l <  r { return Some(-1) }
        return None
      },
      _ => None
    },
    Value::Array(l) => match right {
      Value::Array(r) => {
        if l == r { return Some(0) }
        return None
      },
      _ => None
    },
    // Value::Function(l) => match right {
    //   Value::Function(r) => {
    //     if l == r { return Some(0) }
    //     return None
    //   },
    //   _ => None
    // },
    _ => None
  }
}

pub fn value_eq(left: &Value, right: &Value) -> bool {
  match value_cmp(left, right) {
    Some(v) => match v {
      0 => true,
      _ => false
    }
    _ => false,
  }
}

pub fn value_gt(left: &Value, right: &Value) -> bool {
  match value_cmp(left, right) {
    Some(v) => match v {
      1 => true,
      _ => false
    }
    _ => false,
  }
}

pub fn value_lt(left: &Value, right: &Value) -> bool {
  match value_cmp(left, right) {
    Some(v) => match v {
      -1 => true,
      _ => false
    }
    _ => false,
  }
}

pub fn value_ge(left: &Value, right: &Value) -> bool {
  match value_cmp(left, right) {
    Some(v) => match v {
      0 => true,
      1 => true,
      _ => false
    }
    _ => false,
  }
}

pub fn value_le(left: &Value, right: &Value) -> bool {
  match value_cmp(left, right) {
    Some(v) => match v {
      -1 => true,
      0 => true,
      _ => false
    }
    _ => false,
  }
}

#[test]
fn value_comparisons() {
  let cases = vec![
    (Value::None, Value::None, Some::<i8>(0)),
    (Value::None, Value::Int(0), None),
    (Value::None, Value::String("".to_string()), None),
    (Value::None, Value::Array(vec![]), None),
    
    (Value::Int(0), Value::Int(0), Some(0)),
    (Value::Int(1), Value::Int(0), Some(1)),
    (Value::Int(0), Value::Int(1), Some(-1)),
    (Value::Int(0), Value::String("".to_string()), None),
    (Value::Int(0), Value::Array(vec![]), None),
    
    (Value::String("".to_string()), Value::String("".to_string()), Some(0)),
    (Value::String(String::from("z").to_string()), Value::String("a".to_string()), Some(1)),
    (Value::String("a".to_string()), Value::String(String::from("z").to_string()), Some(-1)),
    (Value::String("".to_string()), Value::Array(vec![]), None),
  ];
  
  for (v0, v1, expected) in cases.iter() {
    let res = value_cmp(&v0, &v1);
    assert_eq!(&res, expected);
  }
}