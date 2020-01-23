use std::collections::HashMap;
use super::{
  Value,
  Scope,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Match {
  pub value: Value,
  pub matched: bool,
  pub is_lr: bool,
  pub start: Scope,
  pub end: Scope
}

impl Match {

  // New top level scope, no vars
  pub fn fail(scope: Scope) -> Self {
    Match {
      value: Value::None,
      matched: false,
      is_lr: false,
      start: scope.clone(),
      end: scope
    }
  }

  pub fn default(scope: Scope) -> Self {
    Match {
      value: Value::None,
      matched: true,
      is_lr: false,
      start: scope.clone(),
      end: scope
    }
  }

  pub fn lr(scope: Scope) -> Self {
    Match {
      value: Value::None,
      matched: false,
      is_lr: true,
      start: scope.clone(),
      end: scope
    }
  }

  pub fn ok(value: Value, start: Scope, end: Scope) -> Self {
    Match {
      value,
      matched: true,
      is_lr: false,
      start,
      end
    }
  }
}
