use std::rc::Rc;
use super::super::super::ast::{
  Pattern
};
use super::super::{
  Scope,
  Match,
  Value,
  Context,
  transform,
  RuntimeError
};

pub fn var(start: Scope, name: String, pattern: &Pattern) -> Result<Match, RuntimeError> {
  let ctx = start.context.clone();
  match transform(start.clone(), pattern) {
    Ok(m) => {
      let _match = m.clone();
      if _match.matched {
        let end = m.end;
        let value = m.value;
        ctx.add_var(name.to_string(), value.clone());
        println!("    add: {} <- {:?} {:?}", name.to_string(), value, &end);
        Ok(Match::ok(value, start, end))
      } else {
        Ok(m)
      }
    },
    Err(e) => Err(e)
  }
}

#[test]
fn var_is_added_on_match() {
  let c = Rc::new(Context::new());
  let s = Scope::from(Rc::new(vec![Value::Int(7)]), c.clone());
  let m = var(s, String::from("x"), &Pattern::Any).unwrap();

  assert_eq!(m.matched, true);
  assert_eq!(m.value, Value::Int(7));
  
  let res = c.get_var(String::from("x"));
  assert_eq!(res, Some(Value::Int(7)));
}
