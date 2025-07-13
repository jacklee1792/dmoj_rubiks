use crate::*;

pub trait PTable: Sized {
    fn compute() -> Self;
    fn eval(&self, c: &Cube) -> i32;
}

pub fn sym8_lr(coord: usize) -> Cube {
    let x2 = Cube::from_repr(0x000, 0x0000, 0x89ab30127456, 0x01234567);
    let lr = Cube::from_repr(0x000, 0x0000, 0xab8956741230, 0x67452301);
    let y2 = Cube::from_repr(0x000, 0x0000, 0x98ba54761032, 0x54761032);
    let mut s = Cube::default();
    if coord & 1 != 0 {
        s = s.compose(&x2);
    }
    if coord & 2 != 0 {
        s = s.compose(&y2);
    }
    if coord & 4 != 0 {
        s = s.compose(&lr);
    }
    s
}

pub fn sym8_y(coord: usize) -> Cube {
    let x2 = Cube::from_repr(0x000, 0x0000, 0x89ab30127456, 0x01234567);
    let y = Cube::from_repr(0x000, 0x0000, 0x8ba947650321, 0x47650321);
    let y2 = Cube::from_repr(0x000, 0x0000, 0x98ba54761032, 0x54761032);
    let mut s = Cube::default();
    if coord & 1 != 0 {
        s = s.compose(&x2);
    }
    if coord & 2 != 0 {
        s = s.compose(&y2);
    }
    if coord & 4 != 0 {
        s = s.compose(&y);
    }
    s
}

pub fn sym16(coord: usize) -> Cube {
    let y = Cube::from_repr(0x000, 0x0000, 0x8ba947650321, 0x47650321);
    let x2 = Cube::from_repr(0x000, 0x0000, 0x89ab30127456, 0x01234567);
    let lr = Cube::from_repr(0x000, 0x0000, 0xab8956741230, 0x67452301);
    let y2 = Cube::from_repr(0x000, 0x0000, 0x98ba54761032, 0x54761032);
    let mut s = Cube::default();
    if coord & 1 != 0 {
        s = s.compose(&x2);
    }
    if coord & 2 != 0 {
        s = s.compose(&y2);
    }
    if coord & 4 != 0 {
        s = s.compose(&y);
    }
    if coord & 8 != 0 {
        s = s.compose(&lr);
    }
    s
}

pub struct PT1 {
    eo_sym8: [(Cube, usize); 2048],
    co_sym8: [(Cube, usize); 2187],
    pt_eflip: [u8; 495 * 336],
    pt_etwist: [u8; 495 * 291],
    pt_eotwist: [u8; 2048 * 291],
}

impl PT1 {
    pub fn coord_eflip(eo_sym8: &[(Cube, usize); 2048], c: &Cube) -> (usize, Cube) {
        use Edge::*;
        let (s, eo_cls) = eo_sym8[c.eo.coord()].clone();
        let c = s.compose_edges(&c).compose_edges(&s.inverse());
        let mask = (1 << FL.coord()) | (1 << FR.coord()) | (1 << BL.coord()) | (1 << BR.coord());
        let eslice = c.ep.index_partial_unordered(mask);
        (eslice * 336 + eo_cls, c)
    }

    pub fn coord_etwist(co_sym8: &[(Cube, usize); 2187], c: &Cube) -> (usize, Cube) {
        use Edge::*;
        let (s, co_cls) = co_sym8[c.co.coord()].clone();
        let c = s.compose(&c).compose(&s.inverse());
        let mask = (1 << FL.coord()) | (1 << FR.coord()) | (1 << BL.coord()) | (1 << BR.coord());
        let eslice = c.ep.index_partial_unordered(mask);
        (eslice * 291 + co_cls, c)
    }

    pub fn coord_eotwist(co_sym8: &[(Cube, usize); 2187], c: &Cube) -> (usize, Cube) {
        let (s, co_cls) = co_sym8[c.co.coord()].clone();
        let c = s.compose(&c).compose(&s.inverse());
        (c.eo.coord() * 291 + co_cls, c)
    }

    pub fn compute_eo_sym8() -> [(Cube, usize); 2048] {
        let mut eo_sym8: [Option<(Cube, usize)>; 2048] = [const { None }; 2048];
        let mut cls = 0;
        for eo in 0..2048 {
            if eo_sym8[eo].is_some() {
                continue;
            }
            let c = Cube {
                eo: EO::from_coord(eo),
                ..Cube::default()
            };
            for s in (0..8).map(sym8_lr) {
                let s1 = s.inverse();
                let c2 = s.compose_edges(&c).compose_edges(&s1);
                assert_eq!(s1.compose_edges(&c2).compose_edges(&s).eo.coord(), eo);
                eo_sym8[c2.eo.coord()] = Some((s1, cls));
            }
            cls += 1;
        }

        eo_sym8.map(Option::unwrap)
    }

    pub fn compute_co_sym8() -> [(Cube, usize); 2187] {
        let mut co_sym8: [Option<(Cube, usize)>; 2187] = [const { None }; 2187];
        let mut cls = 0;
        for co in 0..2187 {
            if co_sym8[co].is_some() {
                continue;
            }
            let c = Cube {
                co: CO::from_coord(co),
                ..Cube::default()
            };
            for s in (0..8).map(sym8_y) {
                let s1 = s.inverse();
                let c2 = s.compose_corners(&c).compose_corners(&s1);
                co_sym8[c2.co.coord()] = Some((s1, cls));
            }
            cls += 1;
        }
        co_sym8.map(Option::unwrap)
    }
}

impl PTable for PT1 {
    fn compute() -> Self {
        let eo_sym8 = Self::compute_eo_sym8();
        let co_sym8 = Self::compute_co_sym8();

        let mut pt_eflip = [None; 495 * 336];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        pt_eflip[0] = Some(0);
        q.push_back((Cube::default(), 0));
        while let Some((c, coord)) = q.pop_front() {
            for m in Move::all() {
                let c = c.apply_move_edges(*m);
                let (coord2, rep) = Self::coord_eflip(&eo_sym8, &c);
                if pt_eflip[coord2].is_none() {
                    pt_eflip[coord2] = Some(pt_eflip[coord].unwrap() + 1);
                    q.push_back((rep, coord2));
                }
            }
        }
        let pt_eflip = pt_eflip.map(Option::unwrap);

        let mut pt_etwist = [None; 495 * 291];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        pt_etwist[0] = Some(0);
        q.push_back((Cube::default(), 0));
        while let Some((c, coord)) = q.pop_front() {
            for m in Move::all() {
                let c = c.apply_move(*m);
                let (coord2, rep) = Self::coord_etwist(&co_sym8, &c);
                if pt_etwist[coord2].is_none() {
                    pt_etwist[coord2] = Some(pt_etwist[coord].unwrap() + 1);
                    q.push_back((rep, coord2));
                }
            }
        }
        let pt_etwist = pt_etwist.map(Option::unwrap);

        let mut pt_eotwist = [None; 2048 * 291];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        pt_eotwist[0] = Some(0);
        q.push_back((Cube::default(), 0));
        while let Some((c, coord)) = q.pop_front() {
            for m in Move::all() {
                let c = c.apply_move(*m);
                let (coord2, rep) = Self::coord_eotwist(&co_sym8, &c);
                if pt_eotwist[coord2].is_none() {
                    pt_eotwist[coord2] = Some(pt_eotwist[coord].unwrap() + 1);
                    q.push_back((rep, coord2));
                }
            }
        }
        let pt_eotwist = pt_eotwist.map(Option::unwrap);

        Self {
            eo_sym8,
            co_sym8,
            pt_eflip,
            pt_etwist,
            pt_eotwist,
        }
    }

    fn eval(&self, c: &Cube) -> i32 {
        let (coord, _) = Self::coord_eflip(&self.eo_sym8, &c);
        let eflip = self.pt_eflip[coord];
        let (coord, _) = Self::coord_etwist(&self.co_sym8, &c);
        let etwist = self.pt_etwist[coord];
        let (coord, _) = Self::coord_eotwist(&self.co_sym8, &c);
        let eotwist = self.pt_eotwist[coord];
        eflip.max(etwist).max(eotwist) as i32
    }
}

pub struct PT2 {
    cp_sym16: [(Cube, usize); 40320],
    pt_eeperm: [u8; 24 * 2768],
}

impl PT2 {
    fn compute_cp_sym16() -> [(Cube, usize); 40320] {
        let mut cp_sym16: [Option<(Cube, usize)>; 40320] = [const { None }; 40320];
        let mut cls = 0;
        for cp in 0..40320 {
            if cp_sym16[cp].is_some() {
                continue;
            }
            let c = Cube {
                cp: Perm::<8>::from_index(cp),
                ..Cube::default()
            };
            for s in (0..16).map(sym16) {
                let s1 = s.inverse();
                let c2 = s.compose(&c).compose(&s1);
                cp_sym16[c2.cp.index()] = Some((s1, cls));
            }
            cls += 1;
        }
        cp_sym16.map(Option::unwrap)
    }

    pub fn coord_eeperm(cp_sym16: &[(Cube, usize)], c: &Cube) -> (usize, Cube) {
        use Edge::*;
        let (conj, cp_cls) = cp_sym16[c.cp.index()].clone();
        let c = conj.compose(&c).compose(&conj.inverse());
        let eslice = c.ep.mask(&[FL, FR, BL, BR].map(|e| e.coord())).index();
        (eslice * 2768 + cp_cls, c)
    }
}

impl PTable for PT2 {
    fn compute() -> Self {
        let cp_sym16 = Self::compute_cp_sym16();

        let mut pt_eeperm = [None; 24 * 2768];
        let mut q: VecDeque<(Cube, usize)> = VecDeque::new();
        pt_eeperm[0] = Some(0);
        q.push_back((Cube::default(), 0));
        while let Some((c, coord)) = q.pop_front() {
            for m in Move::drud_moveset() {
                let c = c.apply_move(*m);
                let (coord2, rep) = Self::coord_eeperm(&cp_sym16, &c);
                if pt_eeperm[coord2].is_none() {
                    pt_eeperm[coord2] = Some(pt_eeperm[coord].unwrap() + 1);
                    q.push_back((rep, coord2));
                }
            }
        }
        let pt_eeperm = pt_eeperm.map(Option::unwrap);

        Self {
            cp_sym16,
            pt_eeperm,
        }
    }

    fn eval(&self, c: &Cube) -> i32 {
        let (coord, _) = Self::coord_eeperm(&self.cp_sym16, &c);
        let eeperm = self.pt_eeperm[coord];
        eeperm as i32
    }
}
