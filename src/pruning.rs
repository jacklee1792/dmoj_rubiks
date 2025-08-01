use std::marker::PhantomData;

use crate::*;
use coord::*;

/// Pruning table for a composite coordinate (R, C), where R is reduced by symmetry.
pub struct PrunTable<R, C>
where
    R: Coord,
    C: Coord,
{
    rsym: SymTable<R>,
    dist: Vec<u8>,
    _r: PhantomData<R>,
    _c: PhantomData<C>,
}

impl<R, C> PrunTable<R, C>
where
    R: Coord,
    C: Coord,
{
    pub fn new(moveset: &'static [Move]) -> Self {
        let rsym = SymTable::new();
        let mut dist = vec![None; rsym.n_conj_classes() * C::N_VALUES];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        dist[0] = Some(0);
        q.push_back((Cube::default(), 0));
        while let Some((a, d)) = q.pop_front() {
            for m in moveset {
                let b = rsym.canonicalize(&a.apply_move(*m));
                let b_coord = Self::coord_no_canonicalize(&rsym, &b);
                if dist[b_coord].is_none() {
                    // This is a new family of coordinates we haven't seen before
                    for s in rsym.self_syms(&b) {
                        let c = C::conj(&b, s);
                        let c_coord = Self::coord_no_canonicalize(&rsym, &c);
                        dist[c_coord] = Some((d + 1) as u8);
                    }
                    q.push_back((b, d + 1));
                }
            }
        }
        let dist = dist.into_iter().map(Option::unwrap).collect::<Vec<_>>();
        Self {
            rsym,
            dist,
            _r: PhantomData,
            _c: PhantomData,
        }
    }

    /// Given a cube, produce a lower bound on the number of moves to reduce the coordinate to 0.
    pub fn eval(&self, c: &Cube) -> i32 {
        let coord = Self::coord(&self.rsym, c);
        self.dist[coord] as i32
    }

    /// Compute the symmetry-reduced composite coordinate.
    fn coord(rsym: &SymTable<R>, c: &Cube) -> usize {
        let c = rsym.canonicalize(c);
        let r = rsym.conj_class(R::index(&c));
        let c = C::index(&c);
        r * C::N_VALUES + c
    }

    /// Compute the symmetry-reduced composite coordinate for a cube which already has its
    /// symmetry-reduced coordinate canonicalized.
    fn coord_no_canonicalize(rsym: &SymTable<R>, c: &Cube) -> usize {
        let r = rsym.conj_class(R::index(&c));
        let c = C::index(&c);
        r * C::N_VALUES + c
    }

    /// Decompose the coordinate into its symmetry-composed and basic component, respectively.
    fn decompose_coord(coord: usize) -> (usize, usize) {
        (coord / C::N_VALUES, coord % C::N_VALUES)
    }
}
