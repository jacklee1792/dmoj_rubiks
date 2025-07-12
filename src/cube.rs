use std::{fmt::Display, num::Wrapping};

use crate::*;

/// CO encodes corner orientation at each edge slot relative to some axis
/// Storage: (2 bits * 8)
/// - CO=00 means correct CO relative to the axis
/// - CO=01 means a clockwise twist is needed to correct CO relative to the axis
/// - CO=10 means a counter-clockwise twist is needed to correct CO relative to the axis
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub struct CO(pub u16);

impl CO {
    pub fn new() -> Self {
        Default::default()
    }

    /// Construct straight from the u16 internal representation, without conversion overhead.
    pub fn from_repr(repr: u16) -> Self {
        Self(repr)
    }

    /// Construct from an association list mapping each piece to its CO, every unspecified piece
    /// is assumed to have solved CO.
    pub fn from_assoc(assoc: &[(Corner, u8)]) -> Self {
        let mut ret = 0;
        for (c, co) in assoc {
            debug_assert!(*co <= 3);
            ret |= (*co as u16) << (c.coord() * 2);
        }
        Self(ret)
    }

    pub fn swizzle(&self, p: Perm<8>) -> Self {
        let mut ret = 0;
        for i in 0..8 {
            let j = p.dest(i);
            ret |= ((self.0 >> (2 * i)) & 3) << (2 * j)
        }
        Self(ret)
    }

    pub fn compose(&self, c: Self) -> Self {
        let mut m = self.0 ^ c.0;
        m &= m << 1;
        let m = Wrapping(((self.0 & c.0) | m) & 0xaaaa);
        Self(((Wrapping(self.0) + Wrapping(c.0)) - m - (m >> 1)).0)
    }
}

impl Display for CO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cos = Vec::new();
        for c in Corner::all() {
            let co = (self.0 >> (c.coord() * 2)) & 3;
            cos.push(format!("{}={}", c, co));
        }
        write!(f, "CO(")?;
        write!(f, "{}", cos.join(" "))?;
        write!(f, ")")
    }
}

// EO encodes edge orientation at each edge slot (1 bit * 12)
// Storage: (1 bit * 12)
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub struct EO(pub u16);

impl EO {
    pub fn new() -> Self {
        Default::default()
    }

    /// Construct straight from the u16 internal representation, without conversion overhead.
    pub fn from_repr(repr: u16) -> Self {
        Self(repr)
    }

    /// Construct from a list specifying which edges are bad.
    pub fn from_bad_edges(edges: &[Edge]) -> Self {
        let mut ret = 0;
        for e in edges {
            ret |= 1 << e.coord();
        }
        Self(ret)
    }

    pub fn swizzle(&self, p: Perm<12>) -> Self {
        let mut ret = 0;
        for i in 0..12 {
            let j = p.dest(i);
            ret |= ((self.0 >> i) & 1) << j
        }
        Self(ret)
    }

    pub fn compose(&self, e: Self) -> Self {
        Self(self.0 ^ e.0)
    }

    pub fn is_bad(&self, edge: Edge) -> bool {
        (self.0 >> edge.coord()) & 1 != 0
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Cube {
    // Edge orientation relative to FB-axis
    pub eo: EO,
    pub co: CO,
    pub ep: Perm<12>,
    pub cp: Perm<8>,
}

impl Cube {
    pub fn new(eo: EO, co: CO, ep: Perm<12>, cp: Perm<8>) -> Self {
        Self { eo, co, ep, cp }
    }

    pub fn from_repr(eo: u16, co: u16, ep: u64, cp: u64) -> Self {
        Self {
            eo: EO::from_repr(eo),
            co: CO::from_repr(co),
            ep: Perm::<12>::from_repr(ep),
            cp: Perm::<8>::from_repr(cp),
        }
    }

    /// The edge at the given slot.
    pub fn edge_at(&self, slot: Edge) -> Edge {
        let coord = self.ep.source(slot.coord());
        Edge::from_coord(coord)
    }

    /// Which slot the given edge is in.
    pub fn find_edge(&self, edge: Edge) -> Edge {
        let coord = self.ep.dest(edge.coord());
        Edge::from_coord(coord)
    }

    pub fn is_eofb(&self) -> bool {
        self.eo.0 == 0
    }

    pub fn is_coud(&self) -> bool {
        self.co.0 == 0
    }

    pub fn is_drud(&self) -> bool {
        if !self.is_eofb() || !self.is_coud() {
            false
        } else {
            use Edge::*;
            let mask =
                (1 << FL.coord()) | (1 << FR.coord()) | (1 << BL.coord()) | (1 << BR.coord());
            let udslice_coord = self.ep.index_partial_unordered(mask);
            udslice_coord == 0
        }
    }

    pub fn is_solved(&self) -> bool {
        self.eo.0 == 0
            && self.co.0 == 0
            && self.ep == Perm::<12>::default()
            && self.cp == Perm::<8>::default()
    }

    pub fn compose_edges(&self, c: &Self) -> (EO, Perm<12>) {
        let eo = self.eo.swizzle(c.ep).compose(c.eo);
        let ep = self.ep.compose(c.ep);
        (eo, ep)
    }

    pub fn compose_corners(&self, c: &Self) -> (CO, Perm<8>) {
        let co = self.co.swizzle(c.cp).compose(c.co);
        let cp = self.cp.compose(c.cp);
        (co, cp)
    }

    pub fn compose(&self, c: &Self) -> Self {
        let (eo, ep) = self.compose_edges(c);
        let (co, cp) = self.compose_corners(c);
        Self { eo, co, ep, cp }
    }

    pub fn apply_move_edges(&self, m: Move) -> Self {
        let (eo, ep) = self.compose_edges(&m.into());
        Self { eo, ep, ..*self }
    }

    pub fn apply_move_corners(&self, m: Move) -> Self {
        let (co, cp) = self.compose_corners(&m.into());
        Self { co, cp, ..*self }
    }

    pub fn apply_move(&self, m: Move) -> Self {
        self.compose(&m.into())
    }
}

impl From<Move> for Cube {
    fn from(m: Move) -> Self {
        use Move::*;
        match m {
            U => Cube::from_repr(0x0000, 0x0000, 0xba9876540321, 0x76540321),
            D => Cube::from_repr(0x0000, 0x0000, 0xba9865473210, 0x65473210),
            F => Cube::from_repr(0x0311, 0x0906, 0xba0476593218, 0x76153204),
            B => Cube::from_repr(0x0c44, 0x9060, 0x26987b543a10, 0x37542610),
            R => Cube::from_repr(0x0000, 0x4281, 0x7a938654b210, 0x46507213),
            L => Cube::from_repr(0x0000, 0x2418, 0xb15876a43290, 0x72643150),
            U2 => Cube::from_repr(0x0000, 0x0000, 0xba9876541032, 0x76541032),
            U3 => Cube::from_repr(0x0000, 0x0000, 0xba9876542103, 0x76542103),
            D2 => Cube::from_repr(0x0000, 0x0000, 0xba9854763210, 0x54763210),
            D3 => Cube::from_repr(0x0000, 0x0000, 0xba9847653210, 0x47653210),
            F2 => Cube::from_repr(0x0000, 0x0000, 0xba8976503214, 0x76013245),
            F3 => Cube::from_repr(0x0311, 0x0906, 0xba4076583219, 0x76403251),
            B2 => Cube::from_repr(0x0000, 0x0000, 0xab9872543610, 0x23546710),
            B3 => Cube::from_repr(0x0c44, 0x9060, 0x62987a543b10, 0x62547310),
            R2 => Cube::from_repr(0x0000, 0x0000, 0x8a9b36547210, 0x06534217),
            R3 => Cube::from_repr(0x0000, 0x4281, 0x3a97b6548210, 0x36570214),
            L2 => Cube::from_repr(0x0000, 0x0000, 0xb9a876143250, 0x71243560),
            L3 => Cube::from_repr(0x0000, 0x2418, 0xb518769432a0, 0x75143620),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_move() {
        use Corner::*;
        use Edge::*;
        let eo = EO::from_bad_edges(&[UB, BL, BR, DB]); // These edges flip under L
        let co = CO::from_assoc(&[(UBL, 2), (DBL, 1), (DBR, 2), (UBR, 1)]); // Standard corner twist effects for L
        let ep = Perm::<12>::from_cycle(&[UB.coord(), BL.coord(), DB.coord(), BR.coord()]);
        let cp = Perm::<8>::from_cycle(&[UBL.coord(), DBL.coord(), DBR.coord(), UBR.coord()]);

        let t = Cube { eo, co, ep, cp };
        println!(
            "Cube::from_repr({:#x}, {:#x}, {:#x}, {:#x})",
            t.eo.0,
            t.co.0,
            t.ep.repr(),
            t.cp.repr()
        );

        let t2 = t.compose(&t);
        println!(
            "Cube::from_repr({:#x}, {:#x}, {:#x}, {:#x})",
            t2.eo.0,
            t2.co.0,
            t2.ep.repr(),
            t2.cp.repr()
        );

        let t3 = t2.compose(&t);
        println!(
            "Cube::from_repr({:#x}, {:#x}, {:#x}, {:#x})",
            t3.eo.0,
            t3.co.0,
            t3.ep.repr(),
            t3.cp.repr()
        );

        // let R: Cube = Move::R.into();
        // println!("{:?}", R);
        // println!("{:?}", R.compose(&R));
        // println!("{:?}", R.compose(&R).compose(&R));
        // println!("{:?}", R.compose(&R).compose(&R).compose(&R));
    }

    #[test]
    fn test_apply_moves() {
        let R: Cube = Move::R.into();
        let U: Cube = Move::U.into();
        let F: Cube = Move::F.into();
        let R3: Cube = Move::R3.into();
        let U3: Cube = Move::U3.into();
        let F3: Cube = Move::F3.into();

        let ru = R.compose(&R).compose(&U).compose(&U);
        println!("rururu: {:?}", ru.compose(&ru).compose(&ru));

        println!("{:#x}", Cube::default().cp.repr())
    }

    #[test]
    fn test_two_jperms_solved() {
        use Move::*;
        let jperm = vec![R, U, R3, F3, R, U, R3, U3, R3, F, R2, U3, R3, U3]
            .into_iter()
            .map(Cube::from)
            .reduce(|acc, m| acc.compose(&m))
            .unwrap();
        assert_eq!(jperm.compose(&jperm), Cube::default());
    }
}
