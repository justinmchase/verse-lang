#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Type {
  None,
  Int,
  String,
  Array,
  Object,
  Pattern,
}
