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

use std::collections::VecDeque;

struct SolveContext<'a, P1, P2> {
    pub best: Option<i32>,
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
    if sc.stack_fin.len() as i32 == fin_len {
        if c.is_solved() {
            let sol_len = sc.stack_dr.len() as i32 + fin_len;
            if let Some(best) = sc.best {
                sc.best = Some(i32::min(best, sol_len));
            } else {
                sc.best = Some(sol_len);
            }
            let alg = sc
                .stack_dr
                .iter()
                .chain(sc.stack_fin.iter())
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("  solution: {} ({})", alg, sol_len);
        }
        return;
    }
    if sc.stack_fin.len() as i32 + sc.pt_fin.eval(&c) > fin_len {
        return;
    }
    for m in Move::drud_moveset() {
        if sc
            .best
            .is_some_and(|best| best <= sc.stack_dr.len() as i32 + fin_len)
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

fn solve_dr<P1, P2>(c: Cube, dr_len: i32, sd: &mut SolveContext<P1, P2>)
where
    P1: PTable,
    P2: PTable,
{
    if sd.stack_dr.len() as i32 == dr_len {
        if c.is_drud() {
            for target_fin in 0..=20 {
                solve_fin(c.clone(), target_fin, sd);
            }
        }
        return;
    }
    if sd.stack_dr.len() as i32 + sd.pt_drud.eval(&c) > dr_len {
        return;
    }
    for m in Move::all() {
        if let Some(last) = sd.stack_dr.last() {
            if last.cancels_with(m) || last.commutes_with(m) && m < last {
                continue;
            }
        }
        sd.stack_dr.push(*m);
        solve_dr(c.apply_move(*m), dr_len, sd);
        sd.stack_dr.pop();
    }
}

fn solve<P1, P2>(c: Cube, sd: &mut SolveContext<P1, P2>)
where
    P1: PTable,
    P2: PTable,
{
    for dr_len in 0..=20 {
        if sd.best.is_some_and(|best| dr_len > best) {
            return;
        }
        eprintln!("check dr len={}", dr_len);
        solve_dr(c.clone(), dr_len, sd);
    }
}

fn main() {
    let alg = "R' U' F L D' R2 D' B' R D' B L U R2 D2 F2 R2 U R2 D' R2 B2 D R' U' F";
    let moves: Vec<Move> = alg.split(" ").map(|m| m.try_into().unwrap()).collect();
    let c = moves
        .into_iter()
        .map(Cube::from)
        .reduce(|c, m| c.compose(&m))
        .unwrap();
    println!("solving: {}", alg);

    let pt_drud = PTFlipUDSlice::compute();
    let pt_fin = PTFinCP::compute();
    let mut sd = SolveContext {
        best: None,
        stack_dr: Vec::new(),
        stack_fin: Vec::new(),
        pt_drud: &pt_drud,
        pt_fin: &pt_fin,
    };

    solve(c, &mut sd);
}
