use crate::coord::*;
use crate::*;

/// Coordinate based on EO of 11 edges [0..2^11)
pub struct CoordEO;

impl Coord for CoordEO {
    const NAME: &'static str = "CoordEO";
    const N_VALUES: usize = 2048;

    fn index(c: &Cube) -> usize {
        c.eo.coord()
    }

    fn rep(c: usize) -> Cube {
        Cube {
            eo: EO::from_coord(c),
            ..Cube::default()
        }
    }

    fn conj(c: &Cube, s: Sym) -> Cube {
        s.conj_edges(c)
    }

    fn syms() -> &'static [Sym] {
        &[
            Sym::UF,
            Sym::UB,
            Sym::DF,
            Sym::DB,
            Sym::UF2,
            Sym::UB2,
            Sym::DF2,
            Sym::DB2,
        ]
    }
}
