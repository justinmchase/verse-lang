use crate::ast::Expression;
use crate::runtime::{Library, Name, Namespace, RuntimeError};
use semver::Version;

pub fn sys() -> Result<Library, RuntimeError> {
  let l = Library::new(&Name::from("sys"), &Version::parse("1.0.0").unwrap());
  l.add_module(&Namespace::parse("hello"), Expression::None)?;

  return Ok(l);
}

#[cfg(test)]
mod tests {
  use super::sys;
  use crate::ast::{Expression, ImportExpression};
  use crate::runtime::resolvers::MemoryResolver;
  use crate::runtime::{Library, Namespace, RuntimeError, Value, Verse};
  use semver::Version;
  use std::rc::Rc;

  #[test]
  fn test_import_sys_module() -> Result<(), RuntimeError> {
    let lib = Library::default();
    lib.add_module(
      &Namespace::from("main"),
      Expression::Import(ImportExpression::Global(
        "sys".into(),
        Version::parse("1.0.0").unwrap(),
        Some(Namespace::parse("hello")),
      )),
    )?;

    // main.v
    // import sys.hello#1
    //

    let r = MemoryResolver::new(vec![Rc::new(sys()?)]);
    let v = Rc::new(Verse::new(Rc::new(lib), vec![Box::new(r)]));
    let res = Verse::run(v, None)?;

    assert_eq!(res, Value::None);
    Ok(())
  }
}
