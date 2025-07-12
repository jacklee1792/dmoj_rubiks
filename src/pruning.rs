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

pub struct PTFlipUDSlice([u8; 495 * 2048]);

impl PTFlipUDSlice {
    pub fn coord_flipudslice(c: &Cube) -> usize {
        use Edge::*;
        let eo_coord = c.eo.0 & ((1 << 11) - 1); // Last bit is fixed by parity
        let mask = (1 << FL.coord()) | (1 << FR.coord()) | (1 << BL.coord()) | (1 << BR.coord());
        let udslice_coord = c.ep.index_partial_unordered(mask);
        udslice_coord * 2048 + (eo_coord as usize)
    }
}

impl PTable for PTFlipUDSlice {
    const NAME: &'static str = "FlipUDSlice";
    const N_ENTRIES: usize = 495 * 2048;

    fn compute() -> Self {
        let mut dist = [21; 495 * 2048];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        let c = Cube::default();
        let coord = Self::coord_flipudslice(&c);
        dist[coord] = 0;
        q.push_back((c, coord));

        while let Some((c, coord)) = q.pop_front() {
            for m in Move::all() {
                let c2 = c.apply_move_edges(*m);
                let coord2 = Self::coord_flipudslice(&c2);
                if dist[coord2] == 21 {
                    dist[coord2] = dist[coord] + 1;
                    q.push_back((c2, coord2));
                }
            }
        }
        Self(dist)
    }

    fn eval(&self, c: &Cube) -> i32 {
        self.eval_coord(Self::coord_flipudslice(&c))
    }

    fn eval_coord(&self, c: usize) -> i32 {
        self.0[c] as i32
    }
}

pub struct PTFinCP {
    cp: [u8; 24 * 40320],
}

impl PTFinCP {
    fn coord_eslice_cp(c: &Cube) -> usize {
        use Edge::*;
        let eslice = c.ep.mask(&[FL, FR, BL, BR].map(|e| e.coord())).index();
        c.cp.index() * 24 + eslice
    }

    fn compute_cp() -> [u8; 24 * 40320] {
        let mut dist = [21; 24 * 40320];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        let c = Cube::default();
        let coord = Self::coord_eslice_cp(&c);
        dist[coord] = 0;
        q.push_back((c, coord));

        while let Some((c, coord)) = q.pop_front() {
            for m in Move::drud_moveset() {
                let c2 = c.apply_move(*m);
                let coord2 = Self::coord_eslice_cp(&c2);
                if dist[coord2] == 21 {
                    dist[coord2] = dist[coord] + 1;
                    q.push_back((c2, coord2));
                }
            }
        }
        dist
    }
}

impl PTable for PTFinCP {
    const NAME: &'static str = "FinCP";
    const N_ENTRIES: usize = 24 * 40320;

    fn compute() -> Self {
        PTFinCP {
            cp: Self::compute_cp(),
        }
    }

    fn eval(&self, c: &Cube) -> i32 {
        self.eval_coord(Self::coord_eslice_cp(c))
    }

    fn eval_coord(&self, c: usize) -> i32 {
        self.cp[c] as i32
    }
}
