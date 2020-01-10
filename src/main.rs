mod ast;
mod runtime;
mod transformation;
mod parser;
mod verse;

use verse::Verse;
use parser::parse;
use ast::{
  Exportable,
  Module,
  Function,
  Expression::{
    // Int,

    Arg,
    Ret,
    Ref,

    Set,
    Add,
    Sub,
  },
  Pattern::{
    Tuple,
    Var,
    Any
  }
};
use runtime::{Value};

fn main() {
  // run with:
  // cargo test -- --nocapture
  let p = parse("
    let test = `($x + $y + $z)
    test({ x: 1, y: 2, z: 3 })
  ");

  let m = Module {
    exports: vec![
      Exportable::Function(Box::new(Function {
        name: "test",
        pattern: Some(Tuple(vec![
          Box::new(Var("x", Box::new(Any))),
          Box::new(Var("y", Box::new(Any))),
          Box::new(Var("z", Box::new(Any)))
        ])),
        body: vec![
          Ret(Some(Box::new(
            Sub((
              Box::new(Add((
                Box::new(Ref("x".to_string())),
                Box::new(Ref("y".to_string())),
              ))),
              Box::new(Ref("z".to_string()))
            ))
          )))
        ]
      }))
    ]
  };
  // let v = Verse::new(m);
  // let res = v.invoke("test", vec![
  //   Value::Int(1),
  //   Value::Int(2),
  //   Value::Int(3)
  // ]);

  // assert_eq!(res, Value::Int(0));
}

#[test]
fn end_to_end() {
  main()
}
