use super::super::{Match, NativePatternHandler, RuntimeError, Scope};

pub fn native(start: Scope, handler: NativePatternHandler) -> Result<Match, RuntimeError> {
  handler(start)
}
