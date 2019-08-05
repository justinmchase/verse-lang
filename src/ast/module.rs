use std::any::Any;
use super::function::Function;

pub enum Exportable {
  Function(Box<Function>)
}

pub struct Module {
  pub exports: Vec<Exportable>
}
