mod ops;
pub mod traits;

pub use ops::{BinaryOperation, Comparator, UnaryOperation};

pub const READ_INT_CALL: &str = "read_int";
pub const PRINT_CALL: &str = "print_int";
pub const RETURN_CALL: &str = "return";
