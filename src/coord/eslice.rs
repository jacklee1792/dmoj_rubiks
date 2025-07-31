use crate::coord::*;
use crate::*;

/// Coordinate based on the locations of the E-slice pieces [0..C(12, 4)).
pub struct CoordESlice;

impl Coord for CoordESlice {
    const NAME: &'static str = "CoordESlice";
    const N_VALUES: usize = 495;

    fn index(c: &Cube) -> usize {
        use Edge::*;
        let mask = (1 << FL.coord()) | (1 << FR.coord()) | (1 << BL.coord()) | (1 << BR.coord());
        c.ep.index_partial_unordered(mask)
    }

    fn rep(_c: usize) -> Cube {
        // Not needed since we never reduce this coordinate by symmetry
        // TODO: do not require this in `Coord` trait, only an extension of it
        Cube::default()
    }

    fn conj(c: &Cube, s: Sym) -> Cube {
        s.conj_edges(c)
    }

    fn syms() -> &'static [Sym] {
        // Not needed here, but eventually we'll want to take the intersection of symmetries
        // between the reduced coordinate and the non-reduced one -- so maybe we can't lift
        // this into another trait like `rep`
        &[]
    }
}
