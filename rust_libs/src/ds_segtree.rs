#![allow(unused_imports)]
use proconio::{fastout, input};

pub struct SegTree<S: Copy + std::fmt::Debug + std::fmt::Display, F: Fn(S, S) -> S> {
    n: usize,      // the size of the original elements
    e: S,          // identity element
    op: F,         //
    size: usize,   // leaf (vec) size. the whole tree size is 2*size.
    nodes: Vec<S>, // 1-indexed (ignore 0)
}

impl<S: Copy + std::fmt::Debug + std::fmt::Display, F: Fn(S, S) -> S> SegTree<S, F> {
    pub fn new(n: usize, e: S, op: F) -> Self {
        Self::new_from_vec(&vec![e; n], e, op)
    }
    pub fn new_from_vec(vals: &Vec<S>, e: S, op: F) -> Self {
        let n = vals.len();
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let mut nodes = vec![e; 2 * size];
        for i in 0..n {
            nodes[size + i] = vals[i];
        }
        for i in (1..size).rev() {
            nodes[i] = op(nodes[i * 2], nodes[i * 2 + 1]);
        }
        SegTree {
            n,
            e,
            op,
            size,
            nodes,
        }
    }
    pub fn update(&mut self, mut ind: usize, x: S) {
        ind += self.size;
        self.nodes[ind] = x;
        while ind > 1 {
            ind >>= 1;
            self.nodes[ind] = (self.op)(self.nodes[ind * 2], self.nodes[ind * 2 + 1]);
        }
    }
    pub fn prod(&self, mut l: usize, mut r: usize) -> S {
        l += self.size;
        r += self.size;
        let mut lv = self.e;
        let mut rv = self.e;
        while l < r {
            if l & 1 == 1 {
                lv = (self.op)(lv, self.nodes[l]);
                l += 1;
            }
            if r & 1 == 1 {
                rv = (self.op)(self.nodes[r - 1], rv);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(lv, rv)
    }
    pub fn print_vals(&self) {
        println!(
            "{:?}",
            self.nodes[self.size..self.size + self.n]
                .iter()
                .collect::<Vec<_>>()
        );
    }
}

// https://atcoder.jp/contests/abc185/tasks/abc185_f
#[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        al: [u64; n],
    }
    let mut seg = SegTree::new_from_vec(&al, 0, |a, b| a ^ b);

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            y: usize,
        }
        if t == 1 {
            seg.update(x - 1, seg.prod(x - 1, x) ^ y as u64);
        } else {
            let ans = seg.prod(x - 1, y);
            println!("{}", ans);
        }
    }
}
