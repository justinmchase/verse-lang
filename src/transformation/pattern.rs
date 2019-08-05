use super::super::runtime::value::Value;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Pattern {
  Default,
  Value(Value),
  Range((Value, Value)),
  Slice((Box<Pattern>, usize, Option<usize>))
}
