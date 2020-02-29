use std::rc::Rc;
use std::cell::RefCell;
use super::super::super::ast::{
  Pattern,
  Expression,
  Module,
};
use super::super::{
  Scope,
  Match,
  Value,
  Function,
  RuntimeError,
  Context,
  Verse,
  Type,
  transform
};

// e.g.
//
// a = "hi" -> 1
// b = "by" -> 2
// c = a | b
//
// 1 = c"hi"
// 2 = c"by"
fn grow(start: Scope, func: &Function) -> Result<Match, RuntimeError> {
  println!("growing...");
  let p = Box::new(func.pattern.clone());
  let e = Box::new(func.expression.clone());
  let s = start.clone();
  let mut mat = Match::fail(start);
  loop {
    s.set_memo(func, &mat);
    match transform(s.clone(), &Pattern::Project(p.clone(), e.clone())) {
      Ok(m) => {
        println!("++GROWN: {:?}", m);
        if !m.matched || m.end <= mat.end {
          break;
        } else {
          mat = m;
        }
      },
      Err(e) => {
        return Err(e);
      }
    }
  }
  Ok(mat)
}

fn rule(start: &Scope, func: &Function) -> Result<Match, RuntimeError> {
  match start.get_memo(func) {
    Some(m) => {
      println!("*MEMO!!*");
      println!("   memo: {:?}", start);
      if m.is_lr {
        match start.peek_stack() {
          Some(f) => {
            // If this function is already at the top of the stack then
            // We have Direct Left Recursion is fine. If it isn't at the top
            // then we have Indirect LR, which is not solvable or supported.
            if &f != func {
              return Err(RuntimeError::IndirectLeftRecursion);
            }
          },
          None => {
            // This should not be possible unless there is a code error
            // in this library.
            return Err(RuntimeError::TransformError);
          }
        }
      }
      println!("--match: {:?}", m);
      Ok(m)
    },
    None => {
      println!("*NO MEMO*");
      start.push_stack(func);
      start.set_memo(func, &Match::lr(start.clone()));
      let s = start.clone();
      let p = Box::new(func.pattern.clone());
      let e = Box::new(func.expression.clone());
      match transform(s.clone(), &Pattern::Project(p, e)) {
        Ok(m) => {
          let mut res = m;
          if res.is_lr {
            match grow(s.clone(), func) {
              Ok(m) => res = m,
              Err(e) => {
                start.pop_stack();
                return Err(e);
              }
            }
          }
          start.pop_stack();
          start.set_memo(func, &res);
          Ok(res)
        },
        Err(e) => {
          start.pop_stack();
          Err(e)
        }
      }
    }
  }
}

pub fn r#ref(start: Scope, name: String) -> Result<Match, RuntimeError> {
  let n = &name;
  let var = start.context.get_var(n.clone());
  match var {
    Some(v) => match v {
      Value::Function(f, ctx) => {
        let s = start.with(ctx);
        rule(&s, &f)
      },
      _ => {
        let pattern = Pattern::Equal(v);
        transform(start, &pattern)
      }
    },
    None => Err(RuntimeError::InvalidReferenceError(name))
  }
}

#[test]
fn ref_projects_function() {
  // a = "hi" -> 1
  // 1 = a
  let s = Scope::new(Rc::new(vec![Value::String(String::from("hi"))]));
  let f = Value::Function(
    Box::new(Function::new(
      &Pattern::Equal(Value::String(String::from("hi"))),
      &Some(Expression::Int(1))
    )),
    Rc::new(Context::new())
  );
  s.context.add_var(String::from("a"), f.clone());

  let m = r#ref(s, String::from("a")).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Int(1));
}

#[test]
fn ref_matches_equal_value() {
  let values = vec![
    Value::None,
    Value::Int(1),
    Value::String(String::from("test")),
    Value::Array(vec![]),
  ];
  for v in values.iter() {
    let s = Scope::new(Rc::new(vec![v.clone()]));
    s.context.add_var(String::from("x"), v.clone());

    let m = r#ref(s, String::from("x")).unwrap();
    assert_eq!(m.matched, true);
    assert_eq!(m.value, *v);
  }
}


#[test]
fn call_direct_left_recursion() {
  
  // Below ast is equivalent to this code:
  //
  // module m {
  //   Int = 0..9
  //   num = Int
  //   sum
  //     = x:sum, y:num -> x + y
  //     | x:num        -> x
  //   
  //   [i:sum] -> i
  // }
  //
  // 6 = m[1, 2, 3]

  let m = Module::new(Expression::Block(vec![
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var(String::from("num"), Box::new(Pattern::Any))),
      Box::new(Expression::Function(
        Box::new(Pattern::Var(String::from("i"), Box::new(Pattern::Type(Type::Int)))), // todo
        Box::new(Some(Expression::Ref(String::from("i"))))
      ))
    )),
    
    Box::new(Expression::Destructure(
      Box::new(Pattern::Var(String::from("sum"), Box::new(Pattern::Any))),
      Box::new(Expression::Function(
        Box::new(Pattern::Or(vec![
          Box::new(Pattern::Project(
            Box::new(Pattern::Then(vec![
              Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Ref(String::from("sum"))))),
              Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Ref(String::from("num"))))),
            ])),
            Box::new(Some(Expression::Add(
              Box::new(Expression::Ref(String::from("x"))),
              Box::new(Expression::Ref(String::from("y"))),
            )))
          )),
          Box::new(Pattern::Project(
            Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Ref(String::from("num"))))),
            Box::new(Some(Expression::Ref(String::from("x"))))
          ))
        ])),
        Box::new(None)
      )),
    )),

    Box::new(Expression::Function(
      Box::new(Pattern::Array(Some(Box::new(Pattern::Var(String::from("i"), Box::new(Pattern::Ref(String::from("sum")))))))),
      Box::new(Some(Expression::Ref(String::from("i")))),
    )),
  ]));
  let mut v = Verse::new(m);
  let res = v.run(Some(Value::Array(vec![
    Value::Int(1),
    Value::Int(2),
    Value::Int(3),
  ])));

  assert_eq!(res, Ok(Value::Int(6)));
}