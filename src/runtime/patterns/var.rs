use crate::ast::Pattern;
use crate::runtime::{transform, Match, RuntimeError, Scope};

pub fn var(start: Scope, name: String, pattern: &Pattern) -> Result<Match, RuntimeError> {
  match transform(start.clone(), pattern) {
    Ok(m) => {
      let _match = m.clone();
      if _match.matched {
        let end = m.end;
        let value = m.value;
        start.context.add_var(name.to_string(), value.clone());
        println!("    add: {} <- {:?} {:?}", name.to_string(), value, &end);
        Ok(Match::ok(value, start, end))
      } else {
        Ok(m)
      }
    }
    Err(e) => Err(e),
  }
}

#[cfg(test)]
mod tests {
  use super::var;
  use crate::ast::Pattern;
  use crate::runtime::{Scope, Value, Verse};
  use std::rc::Rc;

  #[test]
  fn var_is_added_on_match() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let s = Scope::from(v.clone(), c.clone(), Rc::new(vec![Value::Int(7)]));
    let m = var(s, String::from("x"), &Pattern::Any).unwrap();

    assert_eq!(m.matched, true);
    assert_eq!(m.value, Value::Int(7));
    let res = c.get_var(String::from("x"));
    assert_eq!(res, Some(Value::Int(7)));
  }
}
