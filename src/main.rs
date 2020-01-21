mod ast;
mod runtime;

use ast::{
  Module,
  Expression::{
    Add,
    Block,
    Call,
    Destructure,
    Literal,
    Ref,
    Return,
    Sub,

  },
  Pattern::{
    And,
    Any,
    Array,
    Project,
    Var,
  }
};
use runtime::{
  Verse,
  Value,
  Value::{Function}
};

fn main() {
  // run with:
  // cargo test -- --nocapture

  // Below ast is equivalent to this code:
  //
  // add: [x, y] -> x + y
  // sub: [x, y] -> x - y
  // return [x, y, z] -> sub(add(x, y), z)
  // 

  let m = Module::new(Block(vec![
    Box::new(Destructure(
      Box::new(Var("add", Box::new(Any))),
      Box::new(Literal(Function(
        Box::new(Array(
          Box::new(And(vec![
            Box::new(Var("x", Box::new(Any))),
            Box::new(Var("y", Box::new(Any))),
          ]))
        )),
        Box::new(Add(
          Box::new(Ref("x")),
          Box::new(Ref("y")),
        ))
      )))
    )),
    Box::new(Destructure(
      Box::new(Var("sub", Box::new(Any))),
      Box::new(Literal(Function(
        Box::new(Array(
          Box::new(And(vec![
            Box::new(Var("x", Box::new(Any))),
            Box::new(Var("y", Box::new(Any))),
          ]))
        )),
        Box::new(Sub(
          Box::new(Ref("x")),
          Box::new(Ref("y")),
        ))
      )))
    )),
    Box::new(Return(Box::new(Literal(Function(
      Box::new(Array(
        Box::new(And(vec![
          Box::new(Var("x", Box::new(Any))),
          Box::new(Var("y", Box::new(Any))),
          Box::new(Var("z", Box::new(Any))),
        ])),
      )),
      Box::new(Call(
        Box::new(Ref("sub")),
        vec![
          Box::new(Call(
            Box::new(Ref("add")),
            vec![
              Box::new(Ref("x")),
              Box::new(Ref("y")),
            ]
          )),
          Box::new(Ref("z"))
        ]
      ))
    )))))
  ]));
  let mut v = Verse::new(m);
  let res = v.run(Value::Array(vec![
    Value::Int(1),
    Value::Int(2),
    Value::Int(3)
  ]));

  assert_eq!(res, Ok(Value::Int(0)));
}

#[test]
fn end_to_end() {
  main()
}
