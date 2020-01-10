use std::collections::HashMap;
use super::value::Value;
use super::super::ast::{
  Expression,
  Pattern,
  Pattern::{
    Any,
    Tuple,
    Var
  }
};

pub struct Scope {
  pub args: Vec<Value>,
  pub vars: HashMap<String, Value>,
  pub stack: Vec<Value>,
  pub ret: Value
}

impl Scope {
  pub fn new(args: Vec<Value>) -> Self {
    Scope {
      args,
      vars: HashMap::new(),
      stack: Vec::new(),
      ret: Value::None
    }
  }
  
  fn op_add(&mut self) {
    let r = self.stack.pop().unwrap();
    let l = self.stack.pop().unwrap();
    match l {
      Value::None => panic!("Cannot add type None"),
      Value::Array(_a) => panic!("Cannot add type Array"),
      Value::Int(li32) => match r {
        Value::Int(ri32) => self.stack.push(Value::Int(li32 + ri32)),
        Value::String(_s) => panic!("Cannot add types Int and String"),
        Value::Array(_a) => panic!("Cannot add types Int and Array"),
        Value::None => panic!("Cannot add types Int and None"),
      },
      Value::String(lstr) => match r {
        Value::String(rstr) => self.stack.push(Value::String(format!("{}{}", lstr, rstr))),
        Value::Int(_i) => panic!("Cannot add types String and Int"),
        Value::Array(_a) => panic!("Cannot add types String and Array"),
        Value::None => panic!("Cannot add types String and None"),
      },
    }
  }

  fn op_sub(&mut self) {
    let r = self.stack.pop().unwrap();
    let l = self.stack.pop().unwrap();
    match l {
      Value::None => panic!("Cannot sub type None"),
      Value::Array(_v) => panic!("Cannot sub type Array"),
      Value::Int(li32) => match r {
        Value::Int(ri32) => self.stack.push(Value::Int(li32 - ri32)),
        Value::String(_s) => panic!("Cannot sub types Int and String"),
        Value::Array(_a) => panic!("Cannot sub types Int and Array"),
        Value::None => panic!("Cannot sub types Int and None"),
      },
      Value::String(_s) => panic!("Cannot sub a String"),
    }
  }

  pub fn r#match(&mut self, pattern: &Pattern) { // -> Result<Value, TransformError> {
    // match pattern {
    //   Any => any(self),
    //   Tuple => tuple(self),
    //   Var => var(self)
    // }
    ()
  }

  pub fn exec(&mut self, expr: &Expression) {
    match expr {
      // Literals
      Expression::Int(i) => {
        self.stack.push(Value::Int(*i));
        println!("  i32({})", *i);
      },

      // Unary expressions
      Expression::Ret(r) => {
        match r {
          Some(e) => {
            self.exec(e);
            self.ret = self.stack.pop().unwrap();
            println!("  ret pop");
          },
          None => {
            self.ret = Value::None;
            println!("  ret");
          }
        };
      },
      Expression::Arg(n) => {
        let v = &self.args[*n];
        self.stack.push(v.clone());
        println!("  arg {:?}", n);
      },
      Expression::Ref(n) => {
        let v = self.vars.get(n).unwrap();
        self.stack.push(v.clone());
        println!("  ref {}", n);
      },

      // Binary expressions
      Expression::Set((n, e)) => {
        self.exec(e);
        let v = self.stack.pop().unwrap();
        self.vars.insert(n.to_string(), v);
        println!("  set {}", n);
      },
      Expression::Add((l, r)) => {
        self.exec(l);
        self.exec(r);
        self.op_add();
        println!("  add");
      },
      Expression::Sub((l, r)) => {
        self.exec(l);
        self.exec(r);
        self.op_sub();
        println!("  sub");
      }
    }
  }
}
