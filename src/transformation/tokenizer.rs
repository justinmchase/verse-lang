use super::super::runtime::value::Value;
use super::pattern::Pattern;

// pub const tokenize: Pattern = Pattern::Default;

pub fn tokenizer() -> Pattern {
  Pattern::Range((
    Value::String("a".to_string()),
    Value::String("z".to_string())
  ))
}
