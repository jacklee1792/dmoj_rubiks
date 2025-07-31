use crate::coord::*;
use crate::*;

/// Coordinate based on CO of 7 corners [0..3^7)
pub struct CoordCO;

impl Coord for CoordCO {
    const N_VALUES: usize = 2187;

    fn index(c: &Cube) -> usize {
        c.co.coord()
    }

    fn rep(c: usize) -> Cube {
        Cube {
            co: CO::from_coord(c),
            ..Cube::default()
        }
    }

    fn conj(c: &Cube, s: Sym) -> Cube {
        s.conj_corners(c)
    }

    fn syms() -> &'static [Sym] {
        &[
            Sym::UF,
            // Sym::UR,
            // Sym::UL,
            Sym::UB,
            Sym::DF,
            // Sym::DR,
            // Sym::DL,
            Sym::DB,
        ]
    }
}
