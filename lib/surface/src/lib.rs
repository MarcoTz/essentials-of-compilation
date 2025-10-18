mod syntax;
pub mod typecheck;
mod uniquify;

pub use syntax::{Block, Expression, Program, Statement, Type};
pub use typecheck::Typecheck;
pub use uniquify::Uniquify;
