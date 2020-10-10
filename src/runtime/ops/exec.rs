use super::super::super::{
  ast::Expression,
  runtime::{
    ops::{
      add, array, block, call, destructure, import, int, none, object, pattern, reference, string,
      subtract, value,
    },
    Context, RuntimeError, Value, Verse,
  },
};
use std::rc::Rc;

pub fn exec(
  verse: Rc<Verse>,
  context: Rc<Context>,
  expr: &Expression,
) -> Result<Value, RuntimeError> {
  println!("  scope: {:?}", context);
  println!("     op: {:?}", expr);
  match expr {
    Expression::Add(l, r) => add(verse, context, l, r),
    Expression::Block(e) => block(verse, context, e),
    Expression::Call(v, args) => call(verse, context, v, args),
    Expression::Destructure(p, e) => destructure(verse, context, p, e),
    Expression::Import(i) => import(verse, context, i),
    Expression::Int(i) => int(verse, context, *i),
    Expression::None => none(verse, context),
    Expression::Ref(name) => reference(verse, context, name.to_string()),
    Expression::String(s) => string(verse, context, s.to_string()),
    Expression::Sub(l, r) => subtract(verse, context, l, r),
    Expression::Array(e) => array(verse, context, e),
    Expression::Object(f) => object(verse, context, f),
    Expression::Value(v) => value(verse, context, v),
    Expression::Pattern(p) => pattern(verse, context, p),
  }
}
