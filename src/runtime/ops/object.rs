use std::rc::Rc;
use std::collections::HashMap;
use super::super::super::ast::{
  Expression,
  FieldExpression,
};
use super::super::{
  Value,
  Type,
  RuntimeError,
  Context,
  Id,
  exec,
};

fn resolve(
  context: Rc<Context>,
  entries: &mut HashMap<String, Value>,
  field: &FieldExpression
) -> Result<Value, RuntimeError> {
  match field {  
    FieldExpression::Ref(n) => {
      match exec(context, &Expression::Ref(n.to_string())) {
        Ok(v) => {
          entries.insert(n.to_string(), v);
          Ok(Value::None)
        },
        Err(e) => Err(e)
      }
    },
    FieldExpression::Set(n, e) => {
      match exec(context.clone(), e) {
        Ok(v) => {
          entries.insert(n.to_string(), v);
          Ok(Value::None)
        },
        Err(e) => Err(e)
      }
    },
    FieldExpression::Spread(e) => {
      match exec(context.clone(), e) {
        Ok(v) => match v {
          Value::Object(_, f) => {
            for (k,v) in f.iter() {
              entries.insert(k.to_string(), v.clone());
            }
            Ok(Value::None)
          },
          _ => Err(RuntimeError::InvalidTypeError(v, Type::Object)),
        },
        Err(e) => Err(e)
      }
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

pub fn object(context: Rc<Context>, fields: &Vec<FieldExpression>) -> Result<Value, RuntimeError> {
  let id = Id::new();
  let mut map = HashMap::new();
  for f in fields {
    match resolve(context.clone(), &mut map, f) {
      Ok(_) => (),
      Err(e) => {
        return Err(e);
      }
    }
  }

  let newFields = Rc::new(map);
  Ok(Value::Object(
    id,
    newFields
  ))
}

#[test]
fn object_returns_object() {
  let s = Rc::new(Context::new());
  let r = object(s, &vec![]).unwrap();
  match r {
    Value::Object(_, _) => (),
    _ => assert!(false)
  }
}

#[test]
fn object_with_field_returns_object_with_value() {
  let s = Rc::new(Context::new());
  let name = String::from("x");
  let expr = Box::new(Expression::Int(7));

  let r = object(s, &vec![FieldExpression::Set(name.clone(), expr)]).unwrap();
  match r {
    Value::Object(_, f) => assert_eq!(Some(&Value::Int(7)), f.get(&name.clone())),
    _ => assert!(false)
  }
}

#[test]
fn object_with_field_ref_returns_object_with_context_value() {
  let x = String::from("x");
  let c = Context::new();
  c.add_var(x.clone(), Value::Int(11));
  let s = Rc::new(c);
  let r = object(s, &vec![FieldExpression::Ref(x.clone())]).unwrap();
  match r {
    Value::Object(_, f) => assert_eq!(&Value::Int(11), f.get(&x.clone()).unwrap()),
    _ => assert!(false)
  }
}

#[test]
fn object_with_spread_returns_new_object_with_same_fields() {
  let ctx0 = Context::new();
  let obj0 = object(Rc::new(ctx0), &vec![FieldExpression::Set(String::from("x"), Box::new(Expression::Int(7)))]).unwrap();

  let ctx1 = Context::new();
  ctx1.add_var(String::from("x"), obj0);

  let obj1 = object(Rc::new(ctx1), &vec![FieldExpression::Spread(Box::new(Expression::Ref(String::from("x"))))]).unwrap();
  match obj1 {
    Value::Object(_, f) => assert_eq!(Some(&Value::Int(7)), f.get(&String::from("x"))),
    _ => assert!(false)
  }
}
