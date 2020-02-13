mod ast;
mod runtime;

use ast::{
  Module,
  Expression,
  Pattern
};
use runtime::{
  Verse,
  Value
};

fn main() {
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

  let m = Module::new(Expression::Block(vec![
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var(String::from("add"), Box::new(Pattern::Any))),
      Box::new(Expression::Function(
        Box::new(Pattern::Array(Some(Box::new(Pattern::And(vec![
          Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
          Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
        ]))))),
        Box::new(Expression::Add(
          Box::new(Expression::Ref(String::from("x"))),
          Box::new(Expression::Ref(String::from("y"))),
        ))
      ))
    )),
    
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var(String::from("sub"), Box::new(Pattern::Any))),
      Box::new(Expression::Function(
        Box::new(Pattern::Array(Some(Box::new(Pattern::And(vec![
          Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
          Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
        ]))))),
        Box::new(Expression::Sub(
          Box::new(Expression::Ref(String::from("x"))),
          Box::new(Expression::Ref(String::from("y"))),
        ))
      ))
    )),

    Box::new(Expression::Function(
      Box::new(Pattern::Array(Some(Box::new(Pattern::And(vec![
        Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
        Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
        Box::new(Pattern::Var(String::from("z"), Box::new(Pattern::Any))),
      ]))))),
      Box::new(Expression::Call(
        Box::new(Expression::Ref(String::from("sub"))),
        Some(Box::new(Expression::Array(vec![
          Box::new(Expression::Call(
            Box::new(Expression::Ref(String::from("add"))),
            Some(Box::new(Expression::Array(vec![
              Box::new(Expression::Ref(String::from("x"))),
              Box::new(Expression::Ref(String::from("y"))),
            ]))),
          )),
          Box::new(Expression::Ref(String::from("z")))
        ])))
      ))
    ))
  ]));
  let mut v = Verse::new(m);
  let res = v.run(Some(Value::Array(vec![
    Value::Int(1),
    Value::Int(2),
    Value::Int(3),
  ])));

  assert_eq!(res, Ok(Value::Int(0)));
}

#[test]
fn end_to_end() {
  main()
}
