mod coord;
mod cube;
mod face;
mod math;
mod mov;
mod perm;
mod piece;
mod pruning;
mod sym;

use coord::*;
use cube::*;
use face::*;
use math::*;
use mov::*;
use perm::*;
use piece::*;
use pruning::*;
use sym::*;

use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

struct Solver<F1, F2>
where
    F1: Fn(&Cube) -> i32,
    F2: Fn(&Cube) -> i32,
{
    pub start: std::time::Instant,
    pub time_limit: std::time::Duration,

    pub best: Option<Vec<Move>>,
    pub stack_dr: Vec<Move>,
    pub stack_fin: Vec<Move>,
    pub eval_drud: F1,
    pub eval_fin: F2,

    pub time_count: usize,
    pub time_over: bool,
}

impl<F1, F2> Solver<F1, F2>
where
    F1: Fn(&Cube) -> i32,
    F2: Fn(&Cube) -> i32,
{
    fn time_over(&mut self) -> bool {
        if !self.time_over && self.time_count % 32 == 0 {
            self.time_over = self.start.elapsed() > self.time_limit - Duration::from_millis(50)
                && self.best.is_some()
        }
        self.time_count += 1;
        self.time_over
    }

    fn solve_fin(&mut self, c: Cube, fin_len: i32) {
        if self.time_over() {
            return;
        }
        if self.stack_fin.len() as i32 == fin_len {
            if c.is_solved() {
                let sol_len = self.stack_dr.len() + fin_len as usize;
                if self.best.as_ref().is_none_or(|best| best.len() > sol_len) {
                    let alg = self
                        .stack_dr
                        .iter()
                        .chain(self.stack_fin.iter())
                        .copied()
                        .collect::<Vec<_>>();
                    eprintln!(
                        "{} ({}) - {:.2}s",
                        alg.iter()
                            .map(|m| m.to_string())
                            .collect::<Vec<_>>()
                            .join(" "),
                        alg.len(),
                        self.start.elapsed().as_secs_f64(),
                    );
                    self.best = Some(alg);
                }
            }
            return;
        }
        if self.stack_fin.len() as i32 + (self.eval_fin)(&c) > fin_len {
            return;
        }
        for m in Move::drud_moveset() {
            if self
                .best
                .as_ref()
                .is_some_and(|best| best.len() <= self.stack_dr.len() + fin_len as usize)
            {
                break;
            }
            if let Some(last) = self.stack_fin.last().or(self.stack_dr.last()) {
                if last.cancels_with(m) || last.commutes_with(m) && m < last {
                    continue;
                }
            }
            self.stack_fin.push(*m);
            self.solve_fin(c.apply_move(*m), fin_len);
            self.stack_fin.pop();
        }
    }

    fn solve_dr(&mut self, c: Cube, dr_len: i32) {
        if self.time_over() {
            return;
        }
        if self.stack_dr.len() as i32 == dr_len {
            if c.is_drud() {
                for target_fin in 0..=13 {
                    self.solve_fin(c.clone(), target_fin);
                }
            }
            return;
        }
        if self.stack_dr.len() as i32 + (self.eval_drud)(&c) > dr_len {
            return;
        }
        for m in Move::all() {
            if let Some(last) = self.stack_dr.last() {
                if last.cancels_with(m) || last.commutes_with(m) && m < last {
                    continue;
                }
            }
            self.stack_dr.push(*m);
            self.solve_dr(c.apply_move(*m), dr_len);
            self.stack_dr.pop();
        }
    }

    fn solve(&mut self, c: Cube) {
        for dr_len in 0..=20 {
            if self
                .best
                .as_ref()
                .is_some_and(|best| dr_len as usize >= best.len())
            {
                return;
            }
            self.solve_dr(c.clone(), dr_len);
        }
    }
}

fn read_cube_net() -> Cube {
    let layout = r#"
                      UBL.0 UB.0 UBR.0
                      UL.0   u    UR.0
                      UFL.0 UF.0 UFR.0
    UBL.2 UL.1 UFL.1  UFL.2 UF.1 UFR.1  UFR.2 UR.1 UBR.1  UBR.2 UB.1 UBL.1
    BL.1   l    FL.1  FL.0   f    FR.0  FR.1   r    BR.1  BR.0   b    BL.0
    DBL.1 DL.1 DFL.2  DFL.1 DF.1 DFR.2  DFR.1 DR.1 DBR.2  DBR.1 DB.1 DBL.2
                      DFL.0 DF.0 DFR.0
                      DL.0   d    DR.0
                      DBL.0 DB.0 DBR.0
    "#
    .lines()
    .map(|line| line.split_whitespace().collect::<Vec<_>>())
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();

    let mut edges: HashMap<Edge, [char; 2]> = HashMap::new();
    let mut corners: HashMap<Corner, [char; 3]> = HashMap::new();
    let mut faces: HashMap<char, Face> = HashMap::new();
    for (i, line) in std::io::stdin().lines().flatten().take(9).enumerate() {
        for (j, c) in line.split_whitespace().enumerate() {
            let c = c.chars().next().unwrap();
            let s = layout[i][j];
            if let Ok(f) = Face::try_from(s) {
                faces.insert(c, f);
                continue;
            }
            let p = s.split(".").nth(0).unwrap();
            let i = s.split(".").nth(1).unwrap().parse::<usize>().unwrap();
            if let Ok(e) = Edge::try_from(p) {
                edges.entry(e).or_default();
                edges.entry(e).and_modify(|s| s[i] = c);
            }
            if let Ok(k) = Corner::try_from(p) {
                corners.entry(k).or_default();
                corners.entry(k).and_modify(|s| s[i] = c);
            }
        }
    }

    let mut ep = [0; 12];
    let mut eo = vec![];
    for (dest, src) in edges {
        let src = src.map(|c| faces[&c]);
        let m = src.iter().min().cloned().unwrap();
        let flip = src.iter().position(|f| *f == m).unwrap();
        if flip != 0 {
            eo.push(dest);
        }

        let src: Edge = (src[0], src[1]).try_into().unwrap();
        ep[src.coord()] = dest.coord();
    }
    let ep = Perm::<12>::from_dests(&ep);
    let eo = EO::from_bad_edges(&eo);

    let mut cp = [0; 8];
    let mut co = vec![];
    for (dest, src) in corners {
        let src = src.map(|c| faces[&c]);
        let m = src.iter().min().cloned().unwrap();
        let twist = src.iter().position(|f| *f == m).unwrap();
        co.push((dest, twist as u8));

        let src: Corner = (src[0], src[1], src[2]).try_into().unwrap();
        cp[src.coord()] = dest.coord();
    }
    let cp = Perm::<8>::from_dests(&cp);
    let co = CO::from_assoc(&co);

    Cube::new(eo, co, ep, cp)
}

fn main() {
    // let cases = vec![
    //     "B2 L2 U2 L2 U' L2 F2 D2 L2 U F2 L2 U' R' U2 B F2 U' R' D2 L D B",
    //     "F2 L2 B2 U B2 R2 D' F2 U' B2 D2 R2 U' B' U2 L2 D' B' D' U' B' L B",
    //     "F2 U F2 U B2 D' R2 F2 U L2 U B2 U' R' U' B D U L2 F L' D' F2",
    //     "L2 D2 L2 B2 F2 D2 B2 L2 D' B2 L2 U2 R' D L F' U L' U' L U R' U'",
    //     "U L2 U2 L2 B2 L2 R2 D F2 D' U2 B2 U' L' B R F' U' R' B2 F U L' U'",
    // ];
    let alg = "B2 R2 D L2 F2 L2 U2 B2 D L2 D' F2 U' B' R U L' B' D R D' L2 B' U'";
    let c = alg
        .split_whitespace()
        .map(|m| Move::try_from(m).unwrap())
        .map(|m| Cube::from(m))
        .reduce(|a, b| a.compose(&b))
        .unwrap();

    // let c = read_cube_net();
    let start = std::time::Instant::now();

    let pt_co = PrunTable::<CoordCO, CoordESlice>::new(Move::all());
    // let pt_eo = PrunTable::<CoordEO, CoordESlice>::new(Move::all());
    let pt_cp = PrunTable::<CoordCP, CoordESliceEP>::new(Move::drud_moveset());
    let pt_ep = PrunTable::<CoordEP, CoordESliceEP>::new(Move::drud_moveset());
    eprintln!("init: {:.2}s", start.elapsed().as_secs_f64());

    let mut s = Solver {
        start,
        time_limit: std::time::Duration::from_secs(1),
        best: None,
        stack_dr: Vec::new(),
        stack_fin: Vec::new(),
        eval_drud: |c: &Cube| pt_co.eval(c),
        eval_fin: |c: &Cube| pt_cp.eval(c).max(pt_ep.eval(c)),
        time_count: 0,
        time_over: false,
    };
    s.solve(c);
    println!(
        "{}",
        s.best
            .unwrap()
            .iter()
            .map(|m| m.to_string())
            .collect::<String>()
    );
}
