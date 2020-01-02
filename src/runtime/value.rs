#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
  None,
  Int(i32),
  String(String),
  Array(Vec<Value>)
}

pub fn value_eq(left: Value, right: Value) -> bool {
  match left {
    Value::None => match right {
      Value::None => true,
      _ => false
    },
    Value::Int(l) => match right {
      Value::Int(r) => l == r,
      _ => false
    },
    Value::String(l) => match right {
      Value::String(r) => l == r,
      _ => false
    },
    Value::Array(l) => match right {
      Value::Array(r) => l == r,
      _ => false
    }
  }
}

pub fn value_gt(left: Value, right: Value) -> bool {
  match left {
    Value::None => false,
    Value::Int(l) => match right {
      Value::Int(r) => l > r,
      _ => false
    },
    Value::String(l) => match right {
      Value::String(r) => l > r,
      _ => false
    },
    Value::Array(_) => false
  }
}

pub fn value_lt(left: Value, right: Value) -> bool {
  match left {
    Value::None => false,
    Value::Int(l) => match right {
      Value::Int(r) => l < r,
      _ => false
    },
    Value::String(l) => match right {
      Value::String(r) => l < r,
      _ => false
    },
    Value::Array(_) => false
  }
}

pub fn value_ge(left: &Value, right: &Value) -> bool {
  match left {
    Value::None => false,
    Value::Int(l) => match right {
      Value::Int(r) => l >= r,
      _ => false
    },
    Value::String(l) => match right {
      Value::String(r) => l >= r,
      _ => false
    },
    Value::Array(_) => false
  }
}

pub fn value_le(left: &Value, right: &Value) -> bool {
  match left {
    Value::None => false,
    Value::Int(l) => match right {
      Value::Int(r) => l <= r,
      _ => false
    },
    Value::String(l) => match right {
      Value::String(r) => l <= r,
      _ => false
    },
    Value::Array(_) => false
  }
}