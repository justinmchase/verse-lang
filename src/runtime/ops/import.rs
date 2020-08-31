use std::rc::Rc;
use crate::ast::ImportExpression;
use crate::runtime::{
  Value,
  RuntimeError,
  Context,
  Verse,
};

pub fn import(verse: Rc<Verse>, context: Rc<Context>, import_expression: &ImportExpression) -> Result<Value, RuntimeError> {
  match import_expression {
    // A reference to an external module
    ImportExpression::Global(name, version, namespace) => Verse::resolve_library(
      verse,
      name,
      version,
      namespace.as_ref()
    ),
  
    // A module relative to the root of the current library
    ImportExpression::Library(namespace) => Verse::resolve_module(
      verse,
      context.get_library(),
      namespace.as_ref()
    ),

    // The main module of the current verse
    ImportExpression::Main => Verse::resolve_main(verse)
  }
}

#[cfg(test)]
mod tests {
  // #[test]
  // fn none_returns_none() {
  //   let s = Rc::new(Context::new());
  //   let r = none(s);
  //   assert_eq!(r, Ok(Value::None));
  // }
}
