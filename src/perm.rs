use core::iter::Iterator;
use std::{fmt::Debug, ops::Add};

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
    pub const fn from_repr(repr: u64) -> Self {
        Self(repr)
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
        let mut unused = 0xffff;
        let mut ans = 0;
        for i in 0..N {
            let j = 1u16.wrapping_shl(self.dest(i) as u32);
            let invs = (unused & (j - 1)).count_ones();
            ans += invs as usize;
            ans *= usize::max(N - 1 - i, 1);
            unused ^= j;
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

    /// Index of this permutation among all permutations, considering only where the given
    /// indices are mapped to. The index of a permutation that keeps all indices in-place is 0.
    pub fn index_partial_unordered(&self, indices: u16) -> usize {
        // let mut img = 0u16;
        // let mut m = indices;
        // let k = indices.count_ones() as usize;
        // while m != 0 {
        //     let i = m.trailing_zeros() as usize;
        //     let j = self.dest(i);
        //     let b = 1 << j;
        //     let below = (indices & (b - 1)).count_ones() as usize;
        //     let j = if indices & b != 0 {
        //         below
        //     } else {
        //         k + j - below
        //     };
        //     img |= 1 << j;
        //     m &= m - 1;
        // }
        
        // Image of the permutation on these indices
        let mut img = 0u16;
        let mut m = indices;
        while m != 0 {
            let i = m.trailing_zeros() as usize;
            img |= 1 << self.dest(i);
            m &= m - 1;
        }

        // Ugly optimization for speed
        // Special case: if the image keeps everything in place, the coordinate should be 0
        let z = (1 << indices.count_ones()) - 1;
        if img == indices {
            img = z
        } else if img == z {
            img = indices;
        }

        let mut ans = 0;
        let mut i = 1;
        while img != 0 {
            let j = img.trailing_zeros() as usize;
            ans += binom(j, i);
            i += 1;
            img &= img - 1;
        }
        ans
    }
    
    /// Mask the permutation, deleting other indices which are not in the mask. Assumes that
    /// the mask and its complement are disjoint, i.e. no elements are permuted between the two.
    pub fn mask<const K: usize>(&self, indices: &[usize; K]) -> Perm<K> {
        let relabel = self.relabel(indices);
        let mut ret = Perm::<K>(0);
        for i in indices {
            ret.set_dest_nomask(relabel[*i] as usize, relabel[self.dest(*i)] as usize);
        }
        ret
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

    #[test]
    fn test_index_partial_unordered() {
        let p = Perm::<4>::from_dests(&[3, 2, 1, 0]);
        assert_ne!(p.index_partial_unordered(0b0011), 0);

        let p = Perm::<4>::from_dests(&[0, 1, 2, 3]);
        assert_eq!(p.index_partial_unordered(0b0011), 0);

        let p = Perm::<4>::from_dests(&[1, 0, 2, 3]);
        assert_eq!(p.index_partial_unordered(0b0011), 0);

        let mut count = [0; 20];
        for i in 0..720 {
            let p = Perm::<6>::from_index(i);
            let j = p.index_partial_unordered(0b010101);
            count[j] += 1;
        }
        for c in count {
            assert_eq!(c, 720 / 20);
        }
    }
}
