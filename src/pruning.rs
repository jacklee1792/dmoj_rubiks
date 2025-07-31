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
    pub fn new() -> Self {
        let rsym = SymTable::new();
        let mut dist = vec![None; rsym.n_conj_classes() * C::N_VALUES];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        dist[0] = Some(0);
        q.push_back((Cube::default(), 0));
        while let Some((a, d)) = q.pop_front() {
            for m in Move::all() {
                let b = rsym.canonicalize(&a.apply_move(*m));
                let b_coord = Self::coord(&rsym, &b);
                if dist[b_coord].is_none() {
                    // This is a new family of coordinates we haven't seen before
                    for s in rsym.self_syms(&b) {
                        let c = C::conj(&b, s);
                        let c_coord = Self::coord(&rsym, &c);
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
}

// pub trait PTable: Sized {
//     fn compute() -> Self;
//     fn eval(&self, c: &Cube) -> i32;
// }

// const N_CO: usize = 2187;
// const N_CO_SYM8: usize = 291;
// const N_EO: usize = 2048;

// pub struct PT1 {
//     // eo_sym8: [(Cube, usize); 2048],
//     co_sym8: SymData<N_CO, N_CO_SYM8>,

//     // pt_eflip: [u8; 495 * 336],
//     // pt_etwist: [u8; 495 * 291],
//     pt_eotwist: Box<[u8; N_EO * N_CO_SYM8]>,
// }

// impl PT1 {
//     // pub fn coord_eflip(eo_sym8: &[(Cube, usize); 2048], c: &Cube) -> (usize, Cube) {
//     //     use Edge::*;
//     //     let (s, eo_cls) = eo_sym8[c.eo.coord()].clone();
//     //     let c = s.compose_edges(&c).compose_edges(&s.inverse());
//     //     let mask = (1 << FL.coord()) | (1 << FR.coord()) | (1 << BL.coord()) | (1 << BR.coord());
//     //     let eslice = c.ep.index_partial_unordered(mask);
//     //     (eslice * 336 + eo_cls, c)
//     // }

//     // pub fn coord_etwist(co_sym8: &[(Cube, usize); 2187], c: &Cube) -> (usize, Cube) {
//     //     use Edge::*;
//     //     let (s, co_cls) = co_sym8[c.co.coord()].clone();
//     //     let c = s.compose(&c).compose(&s.inverse());
//     //     let mask = (1 << FL.coord()) | (1 << FR.coord()) | (1 << BL.coord()) | (1 << BR.coord());
//     //     let eslice = c.ep.index_partial_unordered(mask);
//     //     (eslice * 291 + co_cls, c)
//     // }

//     // Rotate cube so that CO is a canonical position (as defined in SymData)
//     pub fn canonicalize_co(&self, c: &Cube) -> Cube {
//         let coord = c.co.coord();
//         let s = self.co_sym8.conj[coord];
//         s.conj(&c)
//     }

//     // EOtwist coord for CO-canonicalized cube
//     pub fn coord_eotwist(&self, c: &Cube) -> usize {
//         let coord = c.co.coord();
//         let cls = self.co_sym8.cls[coord];
//         c.eo.coord() * N_CO_SYM8 + cls
//     }

//     // pub fn compute_eo_sym8() -> [(Cube, usize); 2048] {
//     //     let mut eo_sym8: [Option<(Cube, usize)>; 2048] = [const { None }; 2048];
//     //     let mut cls = 0;
//     //     for eo in 0..2048 {
//     //         if eo_sym8[eo].is_some() {
//     //             continue;
//     //         }
//     //         let c = Cube {
//     //             eo: EO::from_coord(eo),
//     //             ..Cube::default()
//     //         };
//     //         for s in (0..8).map(sym8_lr) {
//     //             let s1 = s.inverse();
//     //             let c2 = s.compose_edges(&c).compose_edges(&s1);
//     //             assert_eq!(s1.compose_edges(&c2).compose_edges(&s).eo.coord(), eo);
//     //             eo_sym8[c2.eo.coord()] = Some((s1, cls));
//     //         }
//     //         cls += 1;
//     //     }

//     //     eo_sym8.map(Option::unwrap)
//     // }

//     fn init_pts(&mut self) {
//         // let eo_sym8 = Self::compute_eo_sym8();
//         let mut pt_eotwist = Box::new([None; N_EO * N_CO_SYM8]);
//         let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
//         pt_eotwist[0] = Some(0);
//         q.push_back((Cube::default(), 0));
//         while let Some((rep, coord)) = q.pop_front() {
//             for m in Move::all() {
//                 // Apply the move, and immediately rotate so CO is canonical
//                 let rep2 = self.canonicalize_co(&rep.apply_move(*m));
//                 let coord2 = self.coord_eotwist(&rep2);
//                 let dist = pt_eotwist[coord].unwrap() + 1;
//                 if pt_eotwist[coord2].is_none() {
//                     // This is a new family of coordinates we haven't seen before
//                     let cls = self.co_sym8.cls[rep2.co.coord()];
//                     for sym in self.co_sym8.self_syms(cls) {
//                         let rep2 = sym.conj_edges(&rep2);
//                         let coord2 = self.coord_eotwist(&rep2);
//                         pt_eotwist[coord2] = Some(dist);
//                     }
//                     q.push_back((rep2, coord2));
//                 }
//             }
//         }
//         self.pt_eotwist = Box::new(pt_eotwist.map(Option::unwrap));

//         // let mut pt_eflip = [None; 495 * 336];
//         // let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
//         // pt_eflip[0] = Some(0);
//         // q.push_back((Cube::default(), 0));
//         // while let Some((c, coord)) = q.pop_front() {
//         //     for m in Move::all() {
//         //         let c = c.apply_move_edges(*m);
//         //         let (coord2, rep) = Self::coord_eflip(&eo_sym8, &c);
//         //         if pt_eflip[coord2].is_none() {
//         //             pt_eflip[coord2] = Some(pt_eflip[coord].unwrap() + 1);
//         //             q.push_back((rep, coord2));
//         //         }
//         //     }
//         // }
//         // let pt_eflip = pt_eflip.map(Option::unwrap);

//         // let mut pt_etwist = [None; 495 * 291];
//         // let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
//         // pt_etwist[0] = Some(0);
//         // q.push_back((Cube::default(), 0));
//         // while let Some((c, coord)) = q.pop_front() {
//         //     for m in Move::all() {
//         //         let c = c.apply_move(*m);
//         //         let (coord2, rep) = Self::coord_etwist(&co_sym8, &c);
//         //         if pt_etwist[coord2].is_none() {
//         //             pt_etwist[coord2] = Some(pt_etwist[coord].unwrap() + 1);
//         //             q.push_back((rep, coord2));
//         //         }
//         //     }
//         // }
//         // let pt_etwist = pt_etwist.map(Option::unwrap);
//     }
// }

// impl PTable for PT1 {
//     fn compute() -> Self {
//         let co_sym8: SymData<N_CO, N_CO_SYM8> = SymData::new(SymCoord {
//             coord_fn: |c| c.co.coord(),
//             rep_fn: |coord| Cube {
//                 co: CO::from_coord(coord),
//                 ..Cube::default()
//             },
//             conj_fn: Sym::conj_corners,
//             n_syms: 8,
//         });
//         let mut s = Self {
//             co_sym8,
//             pt_eotwist: Box::new([0; N_EO * N_CO_SYM8]),
//         };
//         s.init_pts();
//         s
//     }

//     fn eval(&self, _c: &Cube) -> i32 {
//         // let (coord, _) = Self::coord_eflip(&self.eo_sym8, &c);
//         // let eflip = self.pt_eflip[coord];
//         // let (coord, _) = Self::coord_etwist(&self.co_sym8, &c);
//         // let etwist = self.pt_etwist[coord];
//         // let (coord, _) = Self::coord_eotwist(&self.co_sym8, &c);
//         // let eotwist = self.pt_eotwist[coord];
//         // // eflip.max(etwist).max(eotwist) as i32
//         // eotwist as i32
//         todo!()
//     }
// }

// pub struct PT2 {
//     cp_sym16: [(Cube, usize); 40320],
//     pt_eeperm: [u8; 24 * 2768],
// }

// impl PT2 {
//     fn compute_cp_sym16() -> [(Cube, usize); 40320] {
//         todo!()
//         // let mut cp_sym16: [Option<(Cube, usize)>; 40320] = [const { None }; 40320];
//         // let mut cls = 0;
//         // for cp in 0..40320 {
//         //     if cp_sym16[cp].is_some() {
//         //         continue;
//         //     }
//         //     let c = Cube {
//         //         cp: Perm::<8>::from_index(cp),
//         //         ..Cube::default()
//         //     };
//         //     for s in (0..16).map(sym16) {
//         //         let s1 = s.inverse();
//         //         let c2 = s.compose(&c).compose(&s1);
//         //         cp_sym16[c2.cp.index()] = Some((s1, cls));
//         //     }
//         //     cls += 1;
//         // }
//         // cp_sym16.map(Option::unwrap)
//     }

//     pub fn coord_eeperm(cp_sym16: &[(Cube, usize)], c: &Cube) -> (usize, Cube) {
//         use Edge::*;
//         let (conj, cp_cls) = cp_sym16[c.cp.index()].clone();
//         let c = conj.compose(&c).compose(&conj.inverse());
//         let eslice = c.ep.mask(&[FL, FR, BL, BR].map(|e| e.coord())).index();
//         (eslice * 2768 + cp_cls, c)
//     }
// }

// impl PTable for PT2 {
//     fn compute() -> Self {
//         let cp_sym16 = Self::compute_cp_sym16();

//         let mut pt_eeperm = [None; 24 * 2768];
//         let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
//         pt_eeperm[0] = Some(0);
//         q.push_back((Cube::default(), 0));
//         while let Some((c, coord)) = q.pop_front() {
//             for m in Move::drud_moveset() {
//                 let c = c.apply_move(*m);
//                 let (coord2, rep) = Self::coord_eeperm(&cp_sym16, &c);
//                 if pt_eeperm[coord2].is_none() {
//                     pt_eeperm[coord2] = Some(pt_eeperm[coord].unwrap() + 1);
//                     q.push_back((rep, coord2));
//                 }
//             }
//         }
//         let pt_eeperm = pt_eeperm.map(Option::unwrap);

//         Self {
//             cp_sym16,
//             pt_eeperm,
//         }
//     }

//     fn eval(&self, c: &Cube) -> i32 {
//         let (coord, _) = Self::coord_eeperm(&self.cp_sym16, &c);
//         let eeperm = self.pt_eeperm[coord];
//         eeperm as i32
//     }
// }
