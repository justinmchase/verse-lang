use std::rc::Rc;
use super::super::super::ast::{
  Pattern
};
use super::super::{
  Scope,
  Match,
  Value,
  transform,
  RuntimeError
};

pub fn var(start: Scope, name: String, pattern: &Pattern) -> Result<Match, RuntimeError> {
  match transform(start.clone(), pattern) {
    Ok(m) => {
      let _match = m.clone();
      if _match.matched {
        let value = m.value.clone();
        Ok(Match::ok(value, start, m.end.add_var(name, m.value)))
      } else {
        Ok(m)
      }
    },
    Err(e) => Err(e)
  }
}

#[test]
fn var_is_added_on_match() {
  let s = Scope::new(Rc::new(vec![Value::Int(7)]));
  let m = var(s, "x".to_string(), &Pattern::Any);

  let _mat = m.unwrap();
  let vars = (*_mat.end.vars).borrow_mut();
  let res = vars.get("x");
  assert_eq!(res, Some(&Value::Int(7)));
}
