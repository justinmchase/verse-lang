use super::function::Function;

pub enum Exportable {
  Function(Box<Function>)
}
