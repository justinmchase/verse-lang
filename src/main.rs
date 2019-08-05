mod ast;
mod runtime;
mod transformation;
mod parser;
mod verse;

use verse::Verse;
use parser::parse;
use ast::{
    function::Function,
    expression::Expression::{
        Int,

        Arg,
        Ret,
        Ref,

        Set,
        Add,
        Sub,
    },
    module::{
        Exportable,
        Module
    }
};
use runtime::value::Value;

fn main() {
    println!("Hello, world!");

    let p = parse("fn test(x, y, z): ret x + y - z end");
    let m = Module {
        exports: vec![
            Exportable::Function(Box::new(Function {
                name: "test",
                parameters: vec![
                    Set(("x".to_string(), Box::new(Arg(0)))),
                    Set(("y".to_string(), Box::new(Arg(1)))),
                    Set(("z".to_string(), Box::new(Arg(2))))
                ],
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
    let v = Verse::new(m);
    let res = v.invoke("test", vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3)
    ]);

    assert_eq!(res, Value::Int(0));
}

#[test]
fn end_to_end() {
    main()
}
