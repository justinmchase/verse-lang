use crate::ast::{Expression, Pattern};
use crate::runtime::{Context, Library, Match, Name, Namespace, RuntimeError, Scope, Type, Value};
use regex::Regex;
use semver::Version;
use std::collections::HashMap;
use std::rc::Rc;

fn Ll(start: Scope) -> Result<Match, RuntimeError> {
  lazy_static! {
    static ref LL: Regex = Regex::new(r"^[\p{Ll}]$").unwrap();
  }

  match start.next() {
    Some(s) => match s.value.clone() {
      Value::String(v) => match LL.is_match(v.as_str()) {
        true => Ok(Match::ok(s.value.clone(), start, s)),
        _ => Ok(Match::fail(start)),
      },
      _ => Ok(Match::fail(start)),
    },
    _ => Ok(Match::fail(start)),
  }
}

pub fn text(
  mut libraries: HashMap<(Name, Version), Rc<Library>>,
) -> Result<HashMap<(Name, Version), Rc<Library>>, RuntimeError> {
  let name = Name::parse("text");
  let version = Version::parse("1.0.0").unwrap();
  let lib = Library::new(&name, &version);
  lib.add_module(
    &Namespace::parse("classes.Ll"),
    Expression::Pattern(Box::new(Pattern::Native(Ll))),
  )?;

  libraries.insert((name, version), Rc::new(lib));
  Ok(libraries)
}

#[cfg(test)]
mod tests {
  use crate::ast::{Expression, ImportExpression, Pattern};
  use crate::runtime::resolvers::MemoryResolver;
  use crate::runtime::{Library, Name, Namespace, RuntimeError, Value, Verse};
  use semver::Version;
  use std::rc::Rc;

  #[test]
  fn test_import_text_module() -> Result<(), RuntimeError> {
    // Ll = import text@1.0.0/classes.Ll
    // Ll("a")
    let main = Expression::Block(vec![
      Box::new(Expression::Destructure(
        Box::new(Pattern::Var(String::from("Ll"), Box::new(Pattern::Any))),
        Box::new(Expression::Import(ImportExpression::Global(
          Name::parse("text"),
          Version::parse("1.0.0").unwrap(),
          Some(Namespace::parse("classes.Ll")),
        ))),
      )),
      Box::new(Expression::Call(
        Box::new(Expression::Ref(String::from("Ll"))),
        Some(Box::new(Expression::Value(Value::String(String::from(
          "a",
        ))))),
      )),
    ]);

    let lib = Library::default();
    lib.add_main(main)?;

    let v = Rc::new(Verse::new(
      Rc::new(lib),
      vec![Box::new(MemoryResolver::core()?)],
    ));

    let res = Verse::run(v, None)?;

    println!("{:?}", res);
    assert_eq!(res, Value::String(String::from("a")));

    Ok(())
  }
}
