mod ast;
mod runtime;

use ast::{
  Exportable,
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
  // test = [x, y, z] -> (x + y) - z
  // 

  let m = Module {
    exports: vec![
      Exportable::Function(Box::new(Function {
        name: "test",
        body: Project(
          Box::new(Array(
            Box::new(And(vec![
              Box::new(Var("x", Box::new(Any))),
              Box::new(Var("y", Box::new(Any))),
              Box::new(Var("z", Box::new(Any))),
            ]))
          )),
          Box::new(Sub(
            Box::new(Add(
              Box::new(Ref("x".to_string())),
              Box::new(Ref("y".to_string())),
            )),
            Box::new(Ref("z".to_string()))
          ))
        )
      }))
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
