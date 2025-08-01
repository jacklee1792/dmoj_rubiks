use crate::*;

// Encoding for the 16 DRUD-preserving symmetries on the cube.
// Layout: m|x|yy (4 bits)
// - y: y rotation
// - x: x2 parity
// - m: LR-mirroring parity
#[derive(Copy, Clone, Debug)]
pub struct Sym(u8);

impl Sym {
    pub const UF: Self = Self(0);
    pub const UR: Self = Self(1);
    pub const UB: Self = Self(2);
    pub const UL: Self = Self(3);
    pub const DB: Self = Self(4);
    pub const DR: Self = Self(5);
    pub const DF: Self = Self(6);
    pub const DL: Self = Self(7);
    pub const UF2: Self = Self(8);
    pub const UR2: Self = Self(9);
    pub const UB2: Self = Self(10);
    pub const UL2: Self = Self(11);
    pub const DB2: Self = Self(12);
    pub const DR2: Self = Self(13);
    pub const DF2: Self = Self(14);
    pub const DL2: Self = Self(15);

    // Symmetry via x2 rotation.
    const X2: Cube = Cube::from_repr(0x000, 0x0000, 0x89ab30127456, 0x01234567);

    // Symmetry via y2 rotation.
    const Y2: Cube = Cube::from_repr(0x000, 0x0000, 0x98ba54761032, 0x54761032);

    // Symmetry via y rotation, not suitable for EO.
    const Y: Cube = Cube::from_repr(0x000, 0x0000, 0x8ba947650321, 0x47650321);

    // Symmetry via mirror across the M slice, not suitable for CO.
    const LR: Cube = Cube::from_repr(0x000, 0x0000, 0xab8956741230, 0x67452301);

    pub fn conjugator(self) -> Cube {
        match self.0 {
            0 => Cube::from_repr(0x0000, 0x0000, 0xba9876543210, 0x76543210),
            1 => Cube::from_repr(0x0000, 0x0000, 0x8ba947650321, 0x47650321),
            2 => Cube::from_repr(0x0000, 0x0000, 0x98ba54761032, 0x54761032),
            3 => Cube::from_repr(0x0000, 0x0000, 0xa98b65472103, 0x65472103),
            4 => Cube::from_repr(0x0000, 0x0000, 0x89ab30127456, 0x01234567),
            5 => Cube::from_repr(0x0000, 0x0000, 0xb89a23016745, 0x30127456),
            6 => Cube::from_repr(0x0000, 0x0000, 0xab8912305674, 0x23016745),
            7 => Cube::from_repr(0x0000, 0x0000, 0x9ab801234567, 0x12305674),
            8 => Cube::from_repr(0x0000, 0x0000, 0xab8956741230, 0x67452301),
            9 => Cube::from_repr(0x0000, 0x0000, 0x9ab845670123, 0x56741230),
            10 => Cube::from_repr(0x0000, 0x0000, 0x89ab74563012, 0x45670123),
            11 => Cube::from_repr(0x0000, 0x0000, 0xb89a67452301, 0x74563012),
            12 => Cube::from_repr(0x0000, 0x0000, 0x98ba10325476, 0x10325476),
            13 => Cube::from_repr(0x0000, 0x0000, 0xa98b21036547, 0x21036547),
            14 => Cube::from_repr(0x0000, 0x0000, 0xba9832107654, 0x32107654),
            15 => Cube::from_repr(0x0000, 0x0000, 0x8ba903214765, 0x03214765),
            _ => panic!("invalid symmetry coordinate: {}", self.0),
        }
    }

    pub fn coord(self) -> usize {
        self.0 as usize
    }

    pub const fn from_coord(coord: usize) -> Self {
        Self(coord as u8)
    }

    pub fn conj_edges(self, c: &Cube) -> Cube {
        let s = self.conjugator();
        let s1 = s.inverse_edges();
        let c2 = s.compose_edges(c).compose_edges(&s1);
        Cube {
            eo: c2.eo,
            ep: c2.ep,
            ..*c
        }
    }

    pub fn conj_corners(self, c: &Cube) -> Cube {
        let s = self.conjugator();
        let s1 = s.inverse_corners();
        let c2 = s.compose_corners(c).compose_corners(&s1);
        Cube {
            co: c2.co,
            cp: c2.cp,
            ..*c
        }
    }

    pub fn conj(self, c: &Cube) -> Cube {
        let s = self.conjugator();
        let s1 = s.inverse();
        s.compose(c).compose(&s1)
    }

    pub fn compose(self, rhs: Sym) -> Self {
        let a = self.0;
        let b = rhs.0;
        let y = if (a & 4) != (a & 8) >> 1 {
            a.wrapping_sub(b) & 3
        } else {
            (a + b) & 3
        };
        Self(((a ^ b) & 0xc) + y)
    }

    pub fn inverse(self) -> Self {
        let a = self.0;
        let y = if (a & 4) != (a & 8) >> 1 {
            a & 3
        } else {
            4u8.wrapping_sub(a) & 3
        };
        Self((a & 0xc) + y)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_inverse() {
        for coord in 0..16 {
            let s = Sym::from_coord(coord);
            let res = s.compose(s.inverse());
            assert_eq!(res.coord(), 0);
        }
    }

    #[test]
    fn test_conj() {
        for coord in 0..16 {
            let s = Sym::from_coord(coord);
            let c = Cube::default();
            assert!(s.conj(&c).is_solved());
        }
    }
}
