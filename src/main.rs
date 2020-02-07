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
  //   [x, y] -> add(x, y)
  // }
  // 
  // m(1, 2)

  let m = Module::new(Expression::Block(vec![
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var("add", Box::new(Pattern::Any))),
      Box::new(Expression::Function(
        Box::new(Pattern::And(vec![
          Box::new(Pattern::Var("x", Box::new(Pattern::Any))),
          Box::new(Pattern::Var("y", Box::new(Pattern::Any))),
        ])),
        Box::new(Expression::Add(
          Box::new(Expression::Ref("x")),
          Box::new(Expression::Ref("y")),
        ))
      ))
    )),

    Box::new(Expression::Function(
      Box::new(Pattern::And(vec![
        Box::new(Pattern::Var("x", Box::new(Pattern::Any))),
        Box::new(Pattern::Var("y", Box::new(Pattern::Any))),
      ])),
      Box::new(Expression::Call(
        Box::new(Expression::Ref("add")),
        vec![
          Box::new(Expression::Ref("x")),
          Box::new(Expression::Ref("y")),
        ]
      ))
    ))
  ]));
  let mut v = Verse::new(m);
  let res = v.run(vec![
    Value::Int(1),
    Value::Int(2),
  ]);

  assert_eq!(res, Ok(Value::Int(3)));
}

#[test]
fn end_to_end() {
  main()
}
