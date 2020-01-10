pub use scope::Scope;
pub use value::{
  Value,
  value_cmp,
  value_eq,
  value_gt,
  value_lt,
  value_ge,
  value_le
};

mod scope;
mod value;
