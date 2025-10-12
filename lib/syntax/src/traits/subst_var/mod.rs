mod lang;

pub trait SubstVar {
    fn subst_var(self, old: &str, new: &str) -> Self;
}
