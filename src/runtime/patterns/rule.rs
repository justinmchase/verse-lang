// use std::rc::Rc;
// use super::super::super::ast::{
//   Pattern,
//   Expression,
//   Module,
// };
// use super::super::{
//   Scope,
//   Match,
//   Value,
//   Type,
//   RuntimeError,
//   transform
// };

// fn grow(start: Scope, pattern: &Pattern) -> Result<Match, RuntimeError> {
//   Ok(Match::default(start))
// }

// pub fn rule(start: Scope, name: String, pattern: &Pattern) -> Result<Match, RuntimeError> {
//   match start.clone().get_memo(pattern) {
//     Some(m) => {
//       if m.is_lr {
//         match start.peek_stack() {
//           Some(r) => if &r != rule {
//             return Err(RuntimeError::TransformError); // better message about indirect left recursion
//           }, 
//           None => {
//             return Err(RuntimeError::TransformError); // A bug in code where the rule is not pushed properly
//           }
//         }
//       }
//       Ok(m)
//     },
//     None => {
//       start.push_stack(rule);
//       start.set_memo(rule, &Match::lr(start.clone()));
//       match transform(start.clone(), &rule.pattern) {
//         Ok(m) => {
//           if m.is_lr {
//             match grow(start.clone(), rule) {
//               Ok(m) => {
//                 start.pop_stack();
//                 start.set_memo(rule, &m);
//                 return Ok(m);
//               },
//               Err(e) => {
//                 start.pop_stack();
//                 return Err(e);
//               }
//             }
//           }
//           start.pop_stack();
//           start.set_memo(rule, &m);
//           Ok(m)
//         },
//         Err(e) => {
//           start.pop_stack();
//           Err(e)
//         }
//       }
//     }
//   }
// }

#[cfg(test)]
mod tests {
  // #[test]
  // fn rule_without_recursion_matches_pattern() {
  //   let p = Pattern::Equal(Value::Int(1));
  //   let r = Rule::new(String::from("x"), &p);
  //   let s = Scope::new(Rc::new(vec![Value::Int(1)]));
  //   let m = rule(s, &r).unwrap();
  //   let matched = m.matched;
  //   let value = m.value;

  //   assert_eq!(matched, true);
  //   assert_eq!(value, Value::Int(1));
  // }

  // #[test]
  // fn rule_multiple_deep_matches_inner_pattern() {
  //   let p0 = Pattern::Equal(Value::Int(1));
  //   let r0 = Rule::new(String::from("x"), &p0);

  //   let p1 = Pattern::Rule(Box::new(r0));
  //   let r1 = Rule::new(String::from("y"), &p1);

  //   let s = Scope::new(Rc::new(vec![Value::Int(1)]));
  //   let m = rule(s, &r1).unwrap();

  //   assert_eq!(m.matched, true);
  //   assert_eq!(m.value, Value::Int(1));
  // }

  // // todo:
  // // Direct Left Recursion Case - success
  // #[test]
  // fn rune_direct_lr() {
      
  //   // module m {
  //   //   num = n:Int -> n
  //   //   add
  //   //     = add, num
  //   //     | num
  //   //
  //   //   main = [a:add] -> a
  //   // }
  //   //
  //   // 0 = m[1, 2, 3]

  //   let m = Module::new(Expression::Block(vec![
  //     Box::new(Expression::Destructure(
  //       Box::new(Pattern::Var(String::from("num"), Box::new(Pattern::Any))),
  //       Box::new(Expression::Function(
  //         Box::new(Pattern::Var(String::from("n"), Box::new(Pattern::Type(Type::Int)))),
  //         Box::new(Expression::Ref(String::from("n")))
  //       ))
  //     )),
      
  //     Box::new(Expression::Destructure(
  //       Box::new(Pattern::Var(String::from("add"), Box::new(Pattern::Any))),
  //       Box::new(Expression::Function(
          
  //         Box::new(Pattern::Array(Some(Box::new(Pattern::And(vec![
  //           Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
  //           Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
  //         ]))))),
  //         Box::new(Expression::Sub(
  //           Box::new(Expression::Ref(String::from("x"))),
  //           Box::new(Expression::Ref(String::from("y"))),
  //         ))
  //       ))
  //     )),

  //     Box::new(Expression::Function(
  //       Box::new(Pattern::Array(Some(Box::new(Pattern::And(vec![
  //         Box::new(Pattern::Var(String::from("x"), Box::new(Pattern::Any))),
  //         Box::new(Pattern::Var(String::from("y"), Box::new(Pattern::Any))),
  //         Box::new(Pattern::Var(String::from("z"), Box::new(Pattern::Any))),
  //       ]))))),
  //       Box::new(Expression::Call(
  //         Box::new(Expression::Ref(String::from("sub"))),
  //         Some(Box::new(Expression::Array(vec![
  //           Box::new(Expression::Call(
  //             Box::new(Expression::Ref(String::from("add"))),
  //             Some(Box::new(Expression::Array(vec![
  //               Box::new(Expression::Ref(String::from("x"))),
  //               Box::new(Expression::Ref(String::from("y"))),
  //             ]))),
  //           )),
  //           Box::new(Expression::Ref(String::from("z")))
  //         ])))
  //       ))
  //     ))
  //   ]));


  //   // let mut v = Verse::new(m);
  //   // let res = v.run(Some(Value::Array(vec![
  //   //   Value::Int(1),
  //   //   Value::Int(2),
  //   //   Value::Int(3),
  //   // ])));

  //   // assert_eq!(res, Ok(Value::Int(0)));

  //   // assert_eq!(m.matched, true);
  //   // assert_eq!(m.value, Value::Int(1));
  // }
  // // Indirect LR case - fail
}
