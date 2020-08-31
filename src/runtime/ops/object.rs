use crate::ast::{Expression, FieldExpression};
use crate::runtime::{exec, Context, Id, RuntimeError, Type, Value, Verse};
use std::collections::HashMap;
use std::rc::Rc;

fn resolve(
  verse: Rc<Verse>,
  context: Rc<Context>,
  entries: &mut HashMap<String, Value>,
  field: &FieldExpression,
) -> Result<Value, RuntimeError> {
  match field {
    FieldExpression::Ref(n) => match exec(verse, context, &Expression::Ref(n.to_string())) {
      Ok(v) => {
        entries.insert(n.to_string(), v);
        Ok(Value::None)
      }
      Err(e) => Err(e),
    },
    FieldExpression::Set(n, e) => match exec(verse, context.clone(), e) {
      Ok(v) => {
        entries.insert(n.to_string(), v);
        Ok(Value::None)
      }
      Err(e) => Err(e),
    },
    FieldExpression::Spread(e) => match exec(verse, context.clone(), e) {
      Ok(v) => match v {
        Value::Object(_, f) => {
          for (k, v) in f.iter() {
            entries.insert(k.to_string(), v.clone());
          }
          Ok(Value::None)
        }
        _ => Err(RuntimeError::InvalidTypeError(v, Type::Object)),
      },
      Err(e) => Err(e),
    },
  }
}

// Three styles supported:

// SET - A field with an expression:
// { x: 1 } = { x: 1 }

// REF - A field name only, which is a ref expr:
// let x = 1
// { x: 1 } = { x }

// SPREAD - Merge an objects keys into this object:
// let x = { y: 1 }
// { y: 1 } = { ...x }

pub fn object(
  verse: Rc<Verse>,
  context: Rc<Context>,
  fields: &Vec<FieldExpression>,
) -> Result<Value, RuntimeError> {
  let id = Id::new();
  let mut map = HashMap::new();
  for f in fields {
    match resolve(verse.clone(), context.clone(), &mut map, f) {
      Ok(_) => (),
      Err(e) => {
        return Err(e);
      }
    }
  }

  let new_fields = Rc::new(map);
  Ok(Value::Object(id, new_fields))
}

#[cfg(test)]
mod tests {
  use super::object;
  use crate::ast::{Expression, FieldExpression};
  use crate::runtime::{Value, Verse};
  use std::rc::Rc;
  #[test]
  fn object_returns_object() {
    let v = Rc::new(Verse::default());
    let s = v.create_context();
    let r = object(v, s, &vec![]).unwrap();
    match r {
      Value::Object(_, _) => (),
      _ => assert!(false),
    }
  }

  #[test]
  fn object_with_field_returns_object_with_value() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let name = String::from("x");
    let expr = Box::new(Expression::Int(7));

    let r = object(v, c, &vec![FieldExpression::Set(name.clone(), expr)]).unwrap();
    match r {
      Value::Object(_, f) => assert_eq!(Some(&Value::Int(7)), f.get(&name.clone())),
      _ => assert!(false),
    }
  }

  #[test]
  fn object_with_field_ref_returns_object_with_context_value() {
    let v = Rc::new(Verse::default());
    let c = v.create_context();
    let x = String::from("x");
    c.add_var(x.clone(), Value::Int(11));
    let r = object(v, c, &vec![FieldExpression::Ref(x.clone())]).unwrap();
    match r {
      Value::Object(_, f) => assert_eq!(&Value::Int(11), f.get(&x.clone()).unwrap()),
      _ => assert!(false),
    }
  }

  #[test]
  fn object_with_spread_returns_new_object_with_same_fields() {
    let v = Rc::new(Verse::default());

    let ctx0 = v.create_context();
    let obj0 = object(
      v.clone(),
      ctx0,
      &vec![FieldExpression::Set(
        String::from("x"),
        Box::new(Expression::Int(7)),
      )],
    )
    .unwrap();

    let ctx1 = v.create_context();
    ctx1.add_var(String::from("x"), obj0);

    let obj1 = object(
      v,
      ctx1,
      &vec![FieldExpression::Spread(Box::new(Expression::Ref(
        String::from("x"),
      )))],
    )
    .unwrap();
    match obj1 {
      Value::Object(_, f) => assert_eq!(Some(&Value::Int(7)), f.get(&String::from("x"))),
      _ => assert!(false),
    }
  }
}
