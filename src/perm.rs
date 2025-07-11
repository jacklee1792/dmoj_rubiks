use core::{iter::Iterator, ops::Index};
use std::{array, fmt::Debug, ops::Add};

use crate::*;

/// A permutation on N <= 16 elements.
/// The representation is a 64-bit integer, bits [4k, 4k+4) encode where the k-th element
/// is mapped to by the permutation.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Perm<const N: usize>(u64);

impl<const N: usize> Default for Perm<N> {
    fn default() -> Self {
        let nbits = N * 4;
        let mask = (1 << nbits) - 1;
        Perm(0xfedcba9876543210 & mask)
    }
}

impl<const N: usize> Add for Perm<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.compose(other)
    }
}

impl<const N: usize> Debug for Perm<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cycles = self
            .cycles()
            .iter()
            .map(|c| {
                let parts = c.iter().map(|i| i.to_string()).collect::<Vec<_>>();
                "(".to_owned() + &parts.join(" ") + ")"
            })
            .collect::<Vec<_>>();
        write!(f, "Perm<{}>({})", N, cycles.join(""))
    }
}

impl<const N: usize> Perm<N> {
    // The identity permutation.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid(&self) -> bool {
        let mut used = [false; N];
        for i in 0..N {
            let j = self.dest(i);
            if used[j] || j >= N {
                return false;
            }
            used[j] = true;
        }
        true
    }

    // The permutation, directly from the internal representation.
    pub fn from_repr(repr: u64) -> Self {
        let ret = Self(repr);
        debug_assert!(ret.is_valid());
        ret
    }

    pub fn repr(&self) -> u64 {
        self.0
    }

    /// The permutation correpsonding to a swap of the two given elements.
    pub fn from_swap(i: usize, j: usize) -> Self {
        debug_assert!(i < N && j < N);
        let mut ret = Self::default();
        ret.set_dest(i, j);
        ret.set_dest(j, i);
        ret
    }

    /// The permutation correpsonding to a cycle of the given elements.
    /// The first index of the cycle is sent to the second index, the second to the third,
    /// and so on.
    pub fn from_cycle(cycle: &[usize]) -> Self {
        let mut ret = Self::default();
        let n = cycle.len();
        for i in 0..n {
            ret.set_dest(cycle[i], cycle[(i + 1) % n]);
        }
        debug_assert!(ret.is_valid());
        ret
    }

    /// The permutation specified by the given array of `dest` values.
    pub fn from_dests(dests: &[usize; N]) -> Self {
        let mut ret = Self(0);
        for (i, dest) in dests.iter().enumerate() {
            ret.set_dest_nomask(i, *dest);
        }
        debug_assert!(ret.is_valid());
        ret
    }

    /// The permutation with the given index, the inverse operation of `index`.
    pub fn from_index(mut index: usize) -> Self {
        // let mut used = [false; N];
        // let mut ans = 0;
        // for i in 0..N {
        //     let ord = used[0..i as usize].iter().filter(|y| !**y).count();
        //     ans += ord * fact(N - 1 - i);
        //     used[i as usize] = true;
        // }
        // ans
        let mut ret = Self(0);
        let mut used = [false; N];
        for i in 0..N {
            let q = fact(N - 1 - i);
            let ord = index / q;
            index = index % q;
            let (x, _) = used
                .iter()
                .enumerate()
                .filter(|(_, used)| !**used)
                .nth(ord)
                .unwrap();
            ret.set_dest_nomask(i, x);
            used[x] = true;
        }
        ret
    }

    // pub fn all() -> impl Iterator<Item = Perm<N>> {
    //     todo!()
    // }

    /// The index that the item at index `i` is sent to
    pub fn dest(&self, i: usize) -> usize {
        (self.0 >> (4 * i) & 0xf) as usize
    }

    fn set_dest(&mut self, i: usize, dest: usize) {
        debug_assert!(i < N && dest < N);
        let mask = u64::MAX - (0xf << (i * 4));
        self.0 = (self.0 & mask) | ((dest as u64) << (i * 4));
    }

    fn set_dest_nomask(&mut self, i: usize, dest: usize) {
        debug_assert!(i < N && dest < N);
        self.0 = self.0 | ((dest as u64) << (i * 4));
    }

    /// The index that the the item at index `i` comes from
    pub fn source(&self, i: usize) -> usize {
        debug_assert!(i < N);
        for j in 0..N {
            if self.dest(j) == i {
                return j;
            }
        }
        unreachable!()
    }

    /// Permute the array using `self`, creating a new array
    pub fn transform<T>(&self, a: &[T; N]) -> [T; N]
    where
        T: Clone,
    {
        let mut b = [const { None }; N];
        for i in 0..N {
            let j = self.dest(i);
            b[j] = Some(a[i].clone());
        }
        b.map(Option::unwrap)
    }

    /// Index of this permutation, some integer in [0, N!). The index of the identity
    /// is 0.
    pub fn index(&self) -> usize {
        let mut used = [false; N];
        let mut ans = 0;
        for i in 0..N {
            let x = self.dest(i);
            let ord = used[0..x as usize].iter().filter(|y| !**y).count();
            ans += ord * fact(N - 1 - i);
            used[x as usize] = true;
        }
        ans
    }

    /// Generate a relabelling of 0..N so that the given K indices map to [0..K), and remaining
    /// indices map to [K..N).
    fn relabel(&self, indices: &[usize]) -> [u8; N] {
        let mut relabel = [None; N];
        for (i, j) in indices.iter().enumerate() {
            relabel[*j] = Some(i as u8);
        }
        let mut next = indices.len() as u8;
        for x in relabel.iter_mut() {
            if x.is_none() {
                *x = Some(next);
                next += 1;
            }
        }
        relabel.map(|x| x.unwrap())
    }

    /// Index of the transformation on the K given indices, some integer in
    /// [0, Perm(N, K)). The index of a permutation which leaves all K given indices
    /// in-place is 0.
    pub fn index_partial(&self, indices: &[usize]) -> usize {
        let relabel = self.relabel(indices);
        let k = indices.len();
        let mut used = [false; N];
        let mut ans = 0;
        for (i, j) in indices.iter().enumerate() {
            let x = relabel[self.dest(*j) as usize];
            let ord = used[0..x as usize].iter().filter(|y| !**y).count();
            ans += ord * perm(N - 1 - i, k - 1 - i);
            used[x as usize] = true;
        }
        ans
    }

    /// Like `index_partial`, but does not consider relative ordering of the indices
    /// passed in. As a consequence, the result is some integer in [0, Binom(N, K)).
    pub fn index_partial_unordered(&self, indices: &[usize]) -> usize {
        let relabel = self.relabel(indices);
        let k = indices.len();
        let mut ans = 0;
        let mut locs = [N as u8; N]; // Avoid dynamic allocation
        for i in indices {
            locs[*i] = relabel[self.dest(*i) as usize]
        }
        locs.sort();
        let mut prev = 0;
        for (i, loc) in locs.into_iter().enumerate().take(k) {
            ans += binom(N - prev, k - i) - binom(N - loc as usize, k - i);
            prev = loc as usize + 1;
        }
        ans
    }

    pub fn compose(&self, rhs: Self) -> Self {
        let mut ret = Self(0);
        for i in 0..N {
            let j = self.dest(i);
            let k = rhs.dest(j);
            ret.set_dest_nomask(i, k);
        }
        ret
    }

    pub fn inverse(&self) -> Self {
        let mut ret = Self(0);
        for i in 0..N {
            let j = self.dest(i);
            ret.set_dest_nomask(j, i);
        }
        ret
    }

    // Disjoint cycles of the permutation which have length at least 2.
    pub fn cycles(&self) -> Vec<Vec<usize>> {
        let mut vis = [false; N];
        let mut ret: Vec<Vec<usize>> = vec![vec![]];
        for i in 0..N {
            if vis[i] {
                continue;
            }
            vis[i] = true;
            let cycle = ret.last_mut().unwrap();
            cycle.push(i);
            let mut j = self.dest(i);
            while j != i {
                vis[j] = true;
                cycle.push(j);
                j = self.dest(j);
            }
            if cycle.len() >= 2 {
                ret.push(vec![]);
            } else {
                cycle.clear();
            }
        }
        if ret.last().unwrap().is_empty() {
            ret.pop();
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_index_from_index() {
        for i in 0..720 {
            let p = Perm::<6>::from_index(i);
            assert_eq!(p.index(), i);
        }
    }
}
