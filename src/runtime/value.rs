#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
  None,
  Int(i32),
  String(String),
  Array(Vec<Value>)
}
