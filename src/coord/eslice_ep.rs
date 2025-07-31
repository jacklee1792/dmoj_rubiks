use crate::coord::*;
use crate::*;

/// Coordinate based on the permutation of the E-slice pieces within the E-slice [0..4!).
pub struct CoordESliceEP;

impl Coord for CoordESliceEP {
    const NAME: &'static str = "CoordESliceEP";
    const N_VALUES: usize = 24;

    fn index(c: &Cube) -> usize {
        use Edge::*;
        c.ep.mask(&[FL, FR, BL, BR].map(|e| e.coord())).index()
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
