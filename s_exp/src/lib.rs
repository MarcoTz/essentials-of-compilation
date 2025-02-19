pub mod errors;
pub mod s_exp;

pub trait Grammar {
    type Symbol: TryFrom<char>;
}
