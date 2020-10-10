use std::rc::Rc;

pub use ops::exec;
pub use patterns::transform;

pub use context::Context;
pub use error::RuntimeError;
pub use id::Id;
pub use library::Library;
pub use name::Name;
pub use namespace::Namespace;
pub use r#match::Match;
pub use r#type::Type;
pub use resolver::Resolver;
pub use scope::Scope;
pub use value::value_eq;
pub use value::Value;
pub use verse::Verse;

// Typedefs
pub type NativeExpressionHandler =
  fn(context: Rc<Context>, value: Value) -> Result<Value, RuntimeError>;

pub type NativePatternHandler = fn(scope: Scope) -> Result<Match, RuntimeError>;

// Sub-modules
mod modules;
mod ops;
mod patterns;
mod resolvers;

// main modules
mod context;
mod error;
mod id;
mod library;
mod r#match;
mod name;
mod namespace;
mod resolver;
mod scope;
mod r#type;
mod value;
mod verse;
