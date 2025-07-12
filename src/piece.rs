use crate::*;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, Ord, Hash)]
pub enum Edge {
    UF,
    UL,
    UB,
    UR,
    DF,
    DL,
    DB,
    DR,
    FR,
    FL,
    BL,
    BR,
}

impl TryFrom<&str> for Edge {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 2 {
            return Err(());
        }
        let c1: Face = s.chars().nth(0).unwrap().try_into().unwrap();
        let c2: Face = s.chars().nth(1).unwrap().try_into().unwrap();
        (c1, c2).try_into()
    }
}

impl TryFrom<(Face, Face)> for Edge {
    type Error = ();

    fn try_from(f: (Face, Face)) -> Result<Self, Self::Error> {
        use Edge::*;
        use Face::*;
        let mut f = [f.0, f.1];
        f.sort();
        match f {
            [U, F] => Ok(UF),
            [U, L] => Ok(UL),
            [U, B] => Ok(UB),
            [U, R] => Ok(UR),
            [D, F] => Ok(DF),
            [D, L] => Ok(DL),
            [D, B] => Ok(DB),
            [D, R] => Ok(DR),
            [F, R] => Ok(FR),
            [F, L] => Ok(FL),
            [B, L] => Ok(BL),
            [B, R] => Ok(BR),
            _ => Err(()),
        }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Edge {
    pub fn coord(&self) -> usize {
        use Edge::*;
        match self {
            UF => 0,
            UL => 1,
            UB => 2,
            UR => 3,
            DF => 4,
            DL => 5,
            DB => 6,
            DR => 7,
            FR => 8,
            FL => 9,
            BL => 10,
            BR => 11,
        }
    }

    pub fn from_coord(coord: usize) -> Self {
        use Edge::*;
        match coord {
            0 => UF,
            1 => UL,
            2 => UB,
            3 => UR,
            4 => DF,
            5 => DL,
            6 => DB,
            7 => DR,
            8 => FR,
            9 => FL,
            10 => BL,
            11 => BR,
            _ => panic!("Invalid edge coordinate: {}", coord),
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.coord() == other.coord()
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coord().partial_cmp(&other.coord())
    }
}

#[derive(Copy, Debug, Clone, Eq, Ord, Hash)]
pub enum Corner {
    UFR,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

impl TryFrom<&str> for Corner {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 3 {
            return Err(());
        }

        let mut chars = s.chars();
        let f1: Face = chars.next().unwrap().try_into().unwrap();
        let f2: Face = chars.next().unwrap().try_into().unwrap();
        let f3: Face = chars.next().unwrap().try_into().unwrap();
        (f1, f2, f3).try_into()
    }
}

impl TryFrom<(Face, Face, Face)> for Corner {
    type Error = ();

    fn try_from(f: (Face, Face, Face)) -> Result<Self, Self::Error> {
        use Corner::*;
        use Face::*;
        let mut f = [f.0, f.1, f.2];
        f.sort();

        match f {
            [U, F, R] => Ok(UFR),
            [U, F, L] => Ok(UFL),
            [U, B, L] => Ok(UBL),
            [U, B, R] => Ok(UBR),
            [D, F, R] => Ok(DFR),
            [D, F, L] => Ok(DFL),
            [D, B, L] => Ok(DBL),
            [D, B, R] => Ok(DBR),
            _ => Err(()),
        }
    }
}

impl Display for Corner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Corner {
    pub fn coord(&self) -> usize {
        use Corner::*;
        match self {
            UFR => 0,
            UFL => 1,
            UBL => 2,
            UBR => 3,
            DFR => 4,
            DFL => 5,
            DBL => 6,
            DBR => 7,
        }
    }

    pub fn all() -> &'static [Corner] {
        use Corner::*;
        &[UFR, UFL, UBL, UBR, DFR, DFL, DBL, DBR]
    }
}

impl PartialEq for Corner {
    fn eq(&self, other: &Self) -> bool {
        self.coord() == other.coord()
    }
}

impl PartialOrd for Corner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.coord().partial_cmp(&other.coord())
    }
}
