use crate::*;

pub trait PTable: Sized {
    fn compute() -> Self;
    fn eval(&self, c: &Cube) -> i32;
}

pub struct PTFlipUDSlice([u8; 495 * 2048]);

impl PTFlipUDSlice {
    pub fn coord_flipudslice(c: &Cube) -> usize {
        use Edge::*;
        let eo_coord = c.eo.0 & ((1 << 11) - 1); // Last bit is fixed by parity
        let udslice_coord =
            c.ep.index_partial_unordered(&[FL.coord(), FR.coord(), BL.coord(), BR.coord()]);
        udslice_coord * 2048 + (eo_coord as usize)
    }
}

impl PTable for PTFlipUDSlice {
    fn compute() -> Self {
        let mut dist = [69; 495 * 2048];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        let c = Cube::default();
        let coord = Self::coord_flipudslice(&c);
        dist[coord] = 0;
        q.push_back((c, coord));

        while let Some((c, coord)) = q.pop_front() {
            for m in Move::all() {
                let c2 = c.apply_move_edges(*m);
                let coord2 = Self::coord_flipudslice(&c2);
                if dist[coord2] == 69 {
                    dist[coord2] = dist[coord] + 1;
                    q.push_back((c2, coord2));
                }
            }
        }
        Self(dist)
    }

    fn eval(&self, c: &Cube) -> i32 {
        self.0[Self::coord_flipudslice(&c)] as i32
    }
}

pub struct PTFinCP {
    cp: [u8; 40320],
}

impl PTFinCP {
    fn compute_cp() -> [u8; 40320] {
        let mut dist = [69; 40320];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        let c = Cube::default();
        let coord = c.cp.index();
        dist[coord] = 0;
        q.push_back((c, coord));

        while let Some((c, coord)) = q.pop_front() {
            for m in Move::drud_moveset() {
                let c2 = c.apply_move_corners(*m);
                let coord2 = c2.cp.index();
                if dist[coord2] == 69 {
                    dist[coord2] = dist[coord] + 1;
                    q.push_back((c2, coord2));
                }
            }
        }
        dist
    }
}

impl PTable for PTFinCP {
    fn compute() -> Self {
        PTFinCP {
            cp: Self::compute_cp(),
        }
    }

    fn eval(&self, c: &Cube) -> i32 {
        self.cp[c.cp.index()] as i32
    }
}
