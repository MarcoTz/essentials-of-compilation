use super::{BlockAccum, Error, ExplicateControl};

impl ExplicateControl for monadic::Atom {
    type Target = core::Atom;

    fn explicate_control(self, _: &mut BlockAccum) -> Result<Self::Target, Error> {
        match self {
            monadic::Atom::Integer(i) => Ok(core::Atom::Integer(i)),
            monadic::Atom::Bool(b) => Ok(core::Atom::Bool(b)),
            monadic::Atom::Variable(v) => Ok(core::Atom::Variable(v)),
        }
    }
}
