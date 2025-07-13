use std::collections::HashMap;

use crate::*;

pub trait PTable: Sized {
    const NAME: &'static str;
    const N_ENTRIES: usize;

    fn compute() -> Self;
    fn eval(&self, c: &Cube) -> i32;
    fn eval_coord(&self, c: usize) -> i32;

    fn report(&self) {
        let mut counts: HashMap<i32, usize> = HashMap::new();
        let mut sum: usize = 0;
        for c in 0..Self::N_ENTRIES {
            let d = self.eval_coord(c);
            counts.entry(d).and_modify(|c| *c += 1).or_default();
            sum += d as usize;
        }

        println!("{}", Self::NAME);
        println!("  mean: {:.3} moves", sum as f64 / Self::N_ENTRIES as f64);
        for d in 0..=21 {
            if let Some(n) = counts.get(&d) {
                let prop = *n as f64 / Self::N_ENTRIES as f64;
                println!("  depth {}: {:.3}%", d, prop * 100.0);
            }
        }
    }
}

pub struct PTFlipUDSlice {
    eo_sym8: [(Cube, usize); 2048],
    pt: [u8; 495 * 336],
}

impl PTFlipUDSlice {
    pub fn coord(eo_sym8: &[(Cube, usize)], c: &Cube) -> usize {
        use Edge::*;
        let (conj, eo_cls) = eo_sym8[c.eo.coord()].clone();
        let c = conj.compose(&c).compose(&conj.inverse());
        let mask = (1 << FL.coord()) | (1 << FR.coord()) | (1 << BL.coord() | (1 << BR.coord()));
        let udslice_coord = c.ep.index_partial_unordered(mask);
        udslice_coord * 336 + eo_cls
    }
}

impl PTable for PTFlipUDSlice {
    const NAME: &'static str = "FlipUDSliceSym";
    const N_ENTRIES: usize = 495 * 336;

    fn compute() -> Self {
        // let y = Cube::from_repr(0x000, 0x0000, 0x8ba947650321, 0x47650321);
        let x2 = Cube::from_repr(0x000, 0x0000, 0x89ab30127456, 0x01234567);
        let lr = Cube::from_repr(0x000, 0x0000, 0xab8956741230, 0x67452301);
        let y2 = Cube::from_repr(0x000, 0x0000, 0x98ba54761032, 0x54761032);
        let mut eo_sym8: [Option<(Cube, usize)>; 2048] = [const { None }; 2048];
        let mut cls = 0;
        for eo in 0..2048 {
            if eo_sym8[eo].is_some() {
                continue;
            }
            let c = Cube{ eo: EO::from_coord(eo), ..Cube::default() };
            for sym in 0..8 {
                let mut s = Cube::default();
                if sym & 1 != 0 {
                    s = s.compose(&x2);
                }
                if sym & 2 != 0 {
                    s = s.compose(&y2);
                }
                if sym & 4 != 0 {
                    s = s.compose(&lr);
                }
                let s1 = s.inverse();
                let c2 = s.compose(&c).compose(&s1);
                eo_sym8[c2.eo.coord()] = Some((s1, cls));
            }
            cls += 1;
        }
        let eo_sym8 = eo_sym8.map(Option::unwrap);

        let mut dist = [21; 495 * 336];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        let c = Cube::default();
        let coord = Self::coord(&eo_sym8, &c);
        dist[coord] = 0;
        q.push_back((c, coord));

        while let Some((c, coord)) = q.pop_front() {
            for m in Move::all() {
                let c2 = c.apply_move_edges(*m);
                let coord2 = Self::coord(&eo_sym8, &c2);
                if dist[coord2] == 21 {
                    dist[coord2] = dist[coord] + 1;
                    q.push_back((c2, coord2));
                }
            }
        }
        Self {
            eo_sym8,
            pt: dist,
        }
    }

    fn eval(&self, c: &Cube) -> i32 {
        self.eval_coord(Self::coord(&self.eo_sym8, &c))
    }

    fn eval_coord(&self, c: usize) -> i32 {
        self.pt[c] as i32
    }
}

pub struct PTFinCP {
    cp_sym16: [(Cube, usize); 40320],
    pt: [u8; 24 * 2768],
}

impl PTFinCP {
    pub fn coord(cp_sym16: &[(Cube, usize)], c: &Cube) -> usize {
        use Edge::*;
        let (conj, cp_cls) = cp_sym16[c.cp.index()].clone();
        let c = conj.compose(&c).compose(&conj.inverse());
        let eslice = c.ep.mask(&[FL, FR, BL, BR].map(|e| e.coord())).index();
        eslice * 2768 + cp_cls
    }
}

impl PTable for PTFinCP {
    const NAME: &'static str = "FinCP";
    const N_ENTRIES: usize = 24 * 2768;

    fn compute() -> Self {
        let y = Cube::from_repr(0x000, 0x0000, 0x8ba947650321, 0x47650321);
        let x2 = Cube::from_repr(0x000, 0x0000, 0x89ab30127456, 0x01234567);
        let lr = Cube::from_repr(0x000, 0x0000, 0xab8956741230, 0x67452301);
        let y2 = Cube::from_repr(0x000, 0x0000, 0x98ba54761032, 0x54761032);
        let mut cp_sym16: [Option<(Cube, usize)>; 40320] = [const { None }; 40320];
        let mut cls = 0;
        for cp in 0..40320 {
            if cp_sym16[cp].is_some() {
                continue;
            }
            let c = Cube{ cp: Perm::<8>::from_index(cp), ..Cube::default() };
            for sym in 0..16 {
                let mut s = Cube::default();
                if sym & 1 != 0 {
                    s = s.compose(&x2);
                }
                if sym & 2 != 0 {
                    s = s.compose(&y2);
                }
                if sym & 4 != 0 {
                    s = s.compose(&y);
                }
                if sym & 8 != 0 {
                    s = s.compose(&lr);
                }
                let s1 = s.inverse();
                let c2 = s.compose(&c).compose(&s1);
                cp_sym16[c2.cp.index()] = Some((s1, cls));
            }
            cls += 1;
        }
        let cp_sym16 = cp_sym16.map(Option::unwrap);

        let mut dist = [21; 24 * 2768];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        let c = Cube::default();
        let coord = Self::coord(&cp_sym16, &c);
        dist[coord] = 0;
        q.push_back((c, coord));

        while let Some((c, coord)) = q.pop_front() {
            for m in Move::drud_moveset() {
                let c2 = c.apply_move(*m);
                let coord2 = Self::coord(&cp_sym16, &c2);
                if dist[coord2] == 21 {
                    dist[coord2] = dist[coord] + 1;
                    q.push_back((c2, coord2));
                }
            }
        }

        Self {
            cp_sym16,
            pt: dist,
        }
    }

    fn eval(&self, c: &Cube) -> i32 {
        self.eval_coord(Self::coord(&self.cp_sym16, c))
    }

    fn eval_coord(&self, c: usize) -> i32 {
        self.pt[c] as i32
    }
}
