use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ByteReg {
    Ah,
    Al,
    Bh,
    Bl,
    Ch,
    Cl,
    Dh,
    Dl,
}

impl fmt::Display for ByteReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ByteReg::Ah => f.write_str("ah"),
            ByteReg::Al => f.write_str("al"),
            ByteReg::Bh => f.write_str("bh"),
            ByteReg::Bl => f.write_str("bl"),
            ByteReg::Ch => f.write_str("ch"),
            ByteReg::Cl => f.write_str("cl"),
            ByteReg::Dh => f.write_str("dh"),
            ByteReg::Dl => f.write_str("dl"),
        }
    }
}
