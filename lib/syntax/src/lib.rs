pub mod lang;
pub mod lang_c;
pub mod lang_mon;
mod ops;
mod traits;
pub mod x86;

pub use ops::{BinaryOperation, Comparator, UnaryOperation};
pub use traits::{
    subst_var::SubstVar,
    used_vars::{UsedVars, fresh_var},
};

pub const READ_INT_CALL: &str = "read_int";
pub const PRINT_CALL: &str = "print_int";
pub const RETURN_CALL: &str = "return";
