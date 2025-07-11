#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Face {
    U,
    D,
    F,
    B,
    R,
    L,
}

impl Face {
    pub fn opposite(&self) -> Self {
        use Face::*;
        match self {
            U => D,
            D => U,
            F => B,
            B => F,
            R => L,
            L => R,
        }
    }
}
