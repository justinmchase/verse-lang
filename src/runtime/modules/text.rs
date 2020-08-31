use std::rc::Rc;
use std::collections::HashMap;
use crate::ast::{
  Expression,
  ImportExpression,
  Pattern,
};
use crate::runtime::{
  Module,
  Value,
  Verse,
};

pub fn text() -> Module {
  Module::new(
    None,
    None,
    Expression::Object(vec![
      // FieldExpression::Set(String::from("Ll"), Box::new(Expression::))
    ])
  )
}

#[cfg(test)]
mod tests {

  #[test]
  fn test_import_text_module() {
    let m = Module::new(
      Some(ImportExpression::Ref(
        String::from("sys"),
        Box::new(ImportExpression::Ref(String::from("text"), None))
      )),
      None,
      Expression::Ref(String::from("text")),
    );
    let mut exports = HashMap::new();
    exports.insert(String::from("sys"), Rc::new(sys()));

    let mut v = Verse::new(Rc::new(m), Some(exports), None);
    let res = v.run(None);

    assert_eq!(res, Ok(Value::None));
  }
}
