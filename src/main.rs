mod cube;
mod face;
mod math;
mod mov;
mod perm;
mod piece;
mod pruning;

use cube::*;
use face::*;
use math::*;
use mov::*;
use perm::*;
use piece::*;
use pruning::*;

use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

struct SolveContext<'a, P1, P2> {
    pub start: std::time::Instant,
    pub time_limit: std::time::Duration,

    pub best: Option<Vec<Move>>,
    pub stack_dr: Vec<Move>,
    pub stack_fin: Vec<Move>,
    pub pt_drud: &'a P1,
    pub pt_fin: &'a P2,
}

fn solve_fin<P1, P2>(c: Cube, fin_len: i32, sc: &mut SolveContext<P1, P2>)
where
    P1: PTable,
    P2: PTable,
{
    if sc.start.elapsed() > sc.time_limit - Duration::from_millis(50) {
        return;
    }
    if sc.stack_fin.len() as i32 == fin_len {
        if c.is_solved() {
            let sol_len = sc.stack_dr.len() + fin_len as usize;
            if sc.best.as_ref().is_none_or(|best| best.len() > sol_len) {
                let alg = sc
                    .stack_dr
                    .iter()
                    .chain(sc.stack_fin.iter())
                    .copied()
                    .collect::<Vec<_>>();
                sc.best = Some(alg);
            }
        }
        return;
    }
    if sc.stack_fin.len() as i32 + sc.pt_fin.eval(&c) > fin_len {
        return;
    }
    for m in Move::drud_moveset() {
        if sc
            .best
            .as_ref()
            .is_some_and(|best| best.len() <= sc.stack_dr.len() + fin_len as usize)
        {
            break;
        }
        if let Some(last) = sc.stack_fin.last().or(sc.stack_dr.last()) {
            if last.cancels_with(m) || last.commutes_with(m) && m < last {
                continue;
            }
        }
        sc.stack_fin.push(*m);
        solve_fin(c.apply_move(*m), fin_len, sc);
        sc.stack_fin.pop();
    }
}

fn solve_dr<P1, P2>(c: Cube, dr_len: i32, sc: &mut SolveContext<P1, P2>)
where
    P1: PTable,
    P2: PTable,
{
    if sc.start.elapsed() > sc.time_limit - Duration::from_millis(50) {
        return;
    }
    if sc.stack_dr.len() as i32 == dr_len {
        if c.is_drud() {
            for target_fin in 0..=13 {
                solve_fin(c.clone(), target_fin, sc);
            }
        }
        return;
    }
    if sc.stack_dr.len() as i32 + sc.pt_drud.eval(&c) > dr_len {
        return;
    }
    for m in Move::all() {
        if let Some(last) = sc.stack_dr.last() {
            if last.cancels_with(m) || last.commutes_with(m) && m < last {
                continue;
            }
        }
        sc.stack_dr.push(*m);
        solve_dr(c.apply_move(*m), dr_len, sc);
        sc.stack_dr.pop();
    }
}

fn solve<P1, P2>(c: Cube, sd: &mut SolveContext<P1, P2>)
where
    P1: PTable,
    P2: PTable,
{
    for dr_len in 0..=20 {
        if sd
            .best
            .as_ref()
            .is_some_and(|best| dr_len as usize >= best.len())
        {
            return;
        }
        solve_dr(c.clone(), dr_len, sd);
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
    let start = std::time::Instant::now();
    let c = read_cube_net();

    let pt_drud = PTFlipUDSlice::compute();
    let pt_fin = PTFinCP::compute();

    // pt_drud.report();
    // pt_fin.report();

    let mut sd = SolveContext {
        start,
        time_limit: std::time::Duration::from_secs(2),
        best: None,
        stack_dr: Vec::new(),
        stack_fin: Vec::new(),
        pt_drud: &pt_drud,
        pt_fin: &pt_fin,
    };

    solve(c, &mut sd);
    println!(
        "{}",
        sd.best
            .unwrap()
            .iter()
            .map(|m| m.to_string())
            .collect::<String>()
    );
}
