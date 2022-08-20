#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::{HashMap, HashSet};

struct BinaryIndexedTree {
    vals: Vec<i64>,
}

impl BinaryIndexedTree {
    pub fn new(n: usize) -> Self {
        Self {
            vals: vec![0; n + 1],
        }
    }

    pub fn from_vals(vals: &[i64]) -> Self {
        let vals = [vec![0], vals.to_vec()].concat();
        Self { vals }
    }

    /// sum of [l; r)
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - self.sum(l)
    }

    /// sum of [0; i)
    fn sum(&self, i: usize) -> i64 {
        let mut ind = i; // 0-ind -> 1-ind (internal)
        let mut s = 0;
        while ind > 0 {
            s += self.vals[ind];
            ind -= ind & ind.wrapping_neg();
        }
        s
    }

    fn add(&mut self, i: usize, x: i64) {
        let mut ind = i + 1;
        let sz = self.vals.len();
        while ind < sz {
            self.vals[ind] += x;
            ind += ind & ind.wrapping_neg();
        }
    }
}

// #[fastout]
fn main() {
    input! {
        //
        n: usize,
        cl: [Usize1; n],
        xl: [Usize1; n],
    }
    let mut bit = BinaryIndexedTree::new(n);
    let mut tento = 0;
    for &x in &xl {
        tento += bit.range_sum((x + 1).min(n), n);
        bit.add(x, 1);
    }

    let mut cxl = vec![vec![]; n];
    for (&c, &x) in cl.iter().zip(xl.iter()) {
        cxl[c].push(x);
    }

    for xs in cxl {
        let comp_map = xs
            .iter()
            .map(|x| *x)
            .collect::<HashSet<_>>()
            .into_iter()
            .sorted()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<_, _>>();
        let xs_comp = xs.iter().map(|x| comp_map[&x]).collect::<Vec<_>>();

        let nn = comp_map.len();
        let mut bit_i = BinaryIndexedTree::new(nn);
        let mut tento_i = 0;
        for &x in &xs_comp {
            tento_i += bit_i.range_sum((x + 1).min(nn), nn);
            bit_i.add(x, 1);
        }
        tento -= tento_i;
    }
    println!("{}", tento);
}
