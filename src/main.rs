mod ast;
mod runtime;

use ast::{
  Module,
  Function,
  Expression::{
    Ref,
    Add,
    Sub,
  },
  Pattern::{
    And,
    Project,
    Array,
    Var,
    Any
  }
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
  // add = [x, y] -> (x + y)
  // sub = [x, y] -> (x - y)
  // z = sub(add(1, 2), 3)
  // 

  let m = Module {
    exports: vec![
      Function::new(
        "test",
        Array(
          Box::new(And(vec![
            Box::new(Var("x", Box::new(Any))),
            Box::new(Var("y", Box::new(Any))),
            Box::new(Var("z", Box::new(Any))),
          ])),
        ),
        Sub(
          Box::new(Add(
            Box::new(Ref("x")),
            Box::new(Ref("y")),
          )),
          Box::new(Ref("z"))
        )
      )
    ]
  };
  let v = Verse::new(m);
  let res = v.invoke("test", Value::Array(vec![
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
