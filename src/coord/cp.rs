use crate::coord::*;
use crate::*;

/// Coordinate based on CP of corners [0..8!)
pub struct CoordCP;

impl Coord for CoordCP {
    const NAME: &'static str = "CoordCP";
    const N_VALUES: usize = 40320;

    fn index(c: &Cube) -> usize {
        c.cp.index()
    }

    fn rep(c: usize) -> Cube {
        Cube {
            cp: Perm::<8>::from_index(c),
            ..Cube::default()
        }
    }

    fn conj(c: &Cube, s: Sym) -> Cube {
        s.conj_corners(c)
    }

    fn syms() -> &'static [Sym] {
        &[
            Sym::UF,
            Sym::UR,
            Sym::UL,
            Sym::UB,
            Sym::DF,
            Sym::DR,
            Sym::DL,
            Sym::DB,
            Sym::UF2,
            Sym::UR2,
            Sym::UL2,
            Sym::UB2,
            Sym::DF2,
            Sym::DR2,
            Sym::DL2,
            Sym::DB2,
        ]
    }
}
