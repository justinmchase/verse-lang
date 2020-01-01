use super::super::runtime::value::Value;
use super::pattern::Pattern;

// pub const tokenize: Pattern = Pattern::Default;

pub fn tokenizer() -> Pattern {
  let new_line = Pattern::Value(Value::String("\n".to_string()));
  let tab = Pattern::Value(Value::String("\t".to_string()));
  let space = Pattern::Value(Value::String(" ".to_string()));

  let whitespace = Pattern::Quantity(
    Box::new(Pattern::Or(vec![
      Box::new(space),
      Box::new(tab),
    ])),
    1,
    None
  );

  let word = Pattern::Quantity(
    Box::new(Pattern::Range(
      Value::String("a".to_string()),
      Value::String("z".to_string())
    )),
    1,
    None
  );

  Pattern::Quantity(
    Box::new(Pattern::Or(vec![
      Box::new(new_line),
      Box::new(whitespace),
      Box::new(word),
      // todo: special characters...
    ])),
    0,
    None
  )
}
