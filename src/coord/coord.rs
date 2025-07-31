use std::marker::PhantomData;

use crate::*;

/// Description of a coordinate which maps to a (right) coset of the cube.
pub trait Coord {
    /// Name of the coordinate, used for debugging purposes.
    const NAME: &'static str;

    /// The number of values of the coordinate.
    const N_VALUES: usize;

    /// Inspect a cube and produce a coordinate associated with the it, some number in
    /// [0, N_VALUES).
    fn index(c: &Cube) -> usize;

    /// Given a coordinate, produce a cube which has that coordinate when `index` is
    /// called on it.
    fn rep(c: usize) -> Cube;

    // Apply a symmetry conjugation to the cube which transforms the coordinate appropriately,
    // for example conjugating corners for a `CO` coordinate.
    fn conj(c: &Cube, s: Sym) -> Cube;

    /// Static listing of symmetries which are applicable to the coordinate, i.e. symmetries
    /// `S` which have the the property: if `x`, `y` are cubes with the same coordinate, then
    /// the conjugates of `x`, `y` under `S` also have the same coordinate.
    fn syms() -> &'static [Sym];
}

/// Symmetry information about a coordinate `C`.
pub struct SymTable<C>
where
    C: Coord,
{
    /// Symmetries which bring the cube with the given coordinate back
    /// to the canonical representative.
    conj: Vec<Sym>,

    /// Conjugacy class of each coordinate.
    cls: Vec<usize>,

    /// Self-symmetries of each conjugacy class, encoded as a bitset.
    ssym: Vec<u16>,

    _c: PhantomData<C>,
}

impl<C> SymTable<C>
where
    C: Coord,
{
    pub fn new() -> Self {
        let mut conj: Vec<Option<Sym>> = vec![None; C::N_VALUES];
        let mut cls: Vec<usize> = vec![0; C::N_VALUES];
        let mut ssym: Vec<u16> = vec![];

        let mut clsno = 0;
        for a_coord in 0..C::N_VALUES {
            let mut s = 0;
            if conj[a_coord].is_some() {
                continue;
            }
            let a = C::rep(a_coord);
            for sym in C::syms() {
                let b = C::conj(&a, *sym);
                let b_coord = C::index(&b);
                conj[b_coord] = Some(sym.inverse());
                cls[b_coord] = clsno;
                if a_coord == b_coord {
                    s |= 1 << sym.coord();
                }
            }
            clsno += 1;
            ssym.push(s);
        }

        let conj = conj.into_iter().map(Option::unwrap).collect();
        Self {
            conj,
            cls,
            ssym,
            _c: PhantomData,
        }
    }

    /// Number of conjugacy classes for the coordinate.
    pub fn n_conj_classes(&self) -> usize {
        self.ssym.len()
    }

    /// Symmetry which brings the coordinate to its conjugacy class representative.
    pub fn canonical_conj(&self, coord: usize) -> Sym {
        self.conj[coord]
    }

    /// Canonicalize the given cube, applying a transformation so that coordinate is the
    /// representative of its conjugacy class.
    pub fn canonicalize(&self, c: &Cube) -> Cube {
        let coord = C::index(c);
        let sym = self.canonical_conj(coord);
        sym.conj(c)
    }

    /// Conjugacy class of the given coordinate.
    pub fn conj_class(&self, coord: usize) -> usize {
        self.cls[coord]
    }

    /// An iterator over self-symmetries of the conjugacy class.
    pub fn self_syms(&self, c: &Cube) -> impl Iterator<Item = Sym> {
        let cls = self.conj_class(C::index(c));
        let mut n = self.ssym[cls];
        std::iter::from_fn(move || {
            if n == 0 {
                None
            } else {
                let pos = n.trailing_zeros();
                n &= n - 1;
                Some(Sym::from_coord(pos as usize))
            }
        })
    }
}
