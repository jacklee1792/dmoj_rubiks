use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, Ord)]
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

#[derive(Copy, Debug, Clone, Eq, Ord)]
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

impl Display for Corner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
