use crate::coord::*;
use crate::*;

/// Coordinate based on the permutation of the non-E-slice pieces in phase 2 [0..8!).
pub struct CoordEP;

impl Coord for CoordEP {
    const NAME: &'static str = "CoordEP";
    const N_VALUES: usize = 40320;

    fn index(c: &Cube) -> usize {
        use Edge::*;
        c.ep.mask(&[UF, UL, UB, UR, DF, DL, DB, DR].map(|e| e.coord()))
            .index()
    }

    fn rep(c: usize) -> Cube {
        use Edge::*;
        let ep = Perm::<8>::from_index(c);
        let mut dests = [0; 12];
        for e in Edge::all() {
            let c = e.coord();
            if *e == FL || *e == FR || *e == BL || *e == BR {
                dests[c] = c;
            } else {
                dests[c] = ep.dest(c);
            }
        }
        Cube {
            ep: Perm::from_dests(&dests),
            ..Cube::default()
        }
    }

    fn conj(c: &Cube, s: Sym) -> Cube {
        s.conj_edges(c)
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
