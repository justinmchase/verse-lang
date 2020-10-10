#![allow(dead_code)] // todo: remove this once past initial test phase

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::rc::Rc;
mod ast;
mod runtime;

use ast::{Expression, Pattern};
use runtime::{Library, Namespace, RuntimeError, Value, Verse};

fn main() -> Result<(), RuntimeError> {
  // run with:
  // cargo test -- --nocapture

  // Below ast is equivalent to this code:
  //
  // module m {
  //   add = [x, y] -> x + y
  //   sub = [x, y] -> x - y
  //   [x, y, z] -> sub[add[x, y], z]
  // }
  //
  // 0 = m[1, 2, 3]

  let main = Expression::Block(vec![
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var(String::from("add"), Box::new(Pattern::Any))),
      Box::new(Expression::Pattern(Box::new(Pattern::Project(
        Box::new(Pattern::Array(Some(Box::new(Pattern::And(vec![
          Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
          Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
        ]))))),
        Box::new(Some(Expression::Add(
          Box::new(Expression::Ref(String::from("x"))),
          Box::new(Expression::Ref(String::from("y"))),
        ))),
      )))),
    )),
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var(String::from("sub"), Box::new(Pattern::Any))),
      Box::new(Expression::Pattern(Box::new(Pattern::Project(
        Box::new(Pattern::Array(Some(Box::new(Pattern::And(vec![
          Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
          Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
        ]))))),
        Box::new(Some(Expression::Sub(
          Box::new(Expression::Ref(String::from("x"))),
          Box::new(Expression::Ref(String::from("y"))),
        ))),
      )))),
    )),
    Box::new(Expression::Pattern(Box::new(Pattern::Project(
      Box::new(Pattern::Array(Some(Box::new(Pattern::And(vec![
        Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
        Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
        Box::new(Pattern::Var(String::from("z"), Box::new(Pattern::Any))),
      ]))))),
      Box::new(Some(Expression::Call(
        Box::new(Expression::Ref(String::from("sub"))),
        Some(Box::new(Expression::Array(vec![
          Box::new(Expression::Call(
            Box::new(Expression::Ref(String::from("add"))),
            Some(Box::new(Expression::Array(vec![
              Box::new(Expression::Ref(String::from("x"))),
              Box::new(Expression::Ref(String::from("y"))),
            ]))),
          )),
          Box::new(Expression::Ref(String::from("z"))),
        ]))),
      ))),
    )))),
  ]);
  let lib = Library::default();
  lib.add_main(main)?;

  let v = Rc::new(Verse::new(Rc::new(lib), vec![]));

  let res = Verse::run(
    v,
    Some(Value::Array(vec![
      Value::Int(1),
      Value::Int(2),
      Value::Int(3),
    ])),
  )?;

  assert_eq!(res, Value::Int(0));

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::main;
  use crate::runtime::RuntimeError;
  #[test]
  fn end_to_end() -> Result<(), RuntimeError> {
    main()
  }
}
