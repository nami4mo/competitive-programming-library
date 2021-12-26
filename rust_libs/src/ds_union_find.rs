#![allow(unused_imports)]
use im_rc::HashMap;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use rand_distr::Uniform;

pub struct UnionFind {
    n: usize,
    parent_or_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            n,
            parent_or_size: vec![-1; n],
        }
    }
    pub fn leader(&mut self, x: usize) -> usize {
        if self.parent_or_size[x] < 0 {
            return x as usize;
        } else {
            self.parent_or_size[x] = self.leader(self.parent_or_size[x] as usize) as i32;
            return self.parent_or_size[x] as usize;
        }
    }
    pub fn merge(&mut self, x: usize, y: usize) -> usize {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let leader = self.leader(x);
        -self.parent_or_size[leader] as usize
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.n];
        for i in 0..self.n {
            let leader = self.leader(i);
            res[leader].push(i);
        }
        res.into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
    }
}

// https://atcoder.jp/contests/abc183/tasks/abc183_f
// #[fastout]
fn main() {
    input! {n: usize, q: usize, cl:[Usize1; n]}
    let mut uf = UnionFind::new(n);
    let mut group_class = vec![HashMap::<usize, usize>::new(); n];
    for i in 0..n {
        group_class[i].insert(cl[i], 1);
    }
    for _ in 0..q {
        input! {t: usize, mut a: Usize1, mut b: Usize1}
        if t == 1 {
            if uf.same(a, b) {
                continue;
            }
            if uf.size(a) < uf.size(b) {
                std::mem::swap(&mut a, &mut b);
            }
            a = uf.leader(a);
            b = uf.leader(b);
            uf.merge(a, b);
            for &(k, v) in &group_class[b].clone() {
                *group_class[a].entry(k).or_insert(0) += v;
            }
        } else {
            let leader = uf.leader(a);
            let ans = match group_class[leader].get(&b) {
                Some(v) => *v,
                None => 0,
            };
            println!("{}", ans);
        }
    }
    pow
}
