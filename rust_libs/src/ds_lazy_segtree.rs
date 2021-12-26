#![allow(unused_imports)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

pub struct SegTree<S, Op, F, Mapping, Composition>
where
    S: Copy + std::fmt::Debug + std::fmt::Display,
    Op: Fn(S, S) -> S,
    F: Copy + std::cmp::PartialEq + std::fmt::Debug + std::fmt::Display,
    Mapping: Fn(F, S) -> S,
    Composition: Fn(F, F) -> F,
{
    n: usize,                 // the size of the original elements
    e: S,                     // identity element
    op: Op,                   //
    size: usize,              // leaf (vec) size. the whole tree size is 2*size.
    data: Vec<S>,             // 1-indexed (ignore 0)
    lazy: Vec<F>,             // 1-indexed (ignore 0)
    mapping: Mapping,         // f(x)
    composition: Composition, // f(g())
    id: F,                    // id(x) -> x
}

impl<S, Op, F, Mapping, Composition> SegTree<S, Op, F, Mapping, Composition>
where
    S: Copy + std::fmt::Debug + std::fmt::Display,
    Op: Fn(S, S) -> S,
    F: Copy + std::cmp::PartialEq + std::fmt::Debug + std::fmt::Display,
    Mapping: Fn(F, S) -> S,
    Composition: Fn(F, F) -> F,
{
    pub fn new(n: usize, e: S, op: Op, id: F, mapping: Mapping, composition: Composition) -> Self {
        Self::new_from_vec(&vec![e; n], e, op, id, mapping, composition)
    }
    pub fn new_from_vec(
        vals: &Vec<S>,
        e: S,
        op: Op,
        id: F,
        mapping: Mapping,
        composition: Composition,
    ) -> Self {
        let n = vals.len();
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let mut data = vec![e; size * 2];
        for i in 0..n {
            data[size + i] = vals[i];
        }
        for i in (1..size).rev() {
            data[i] = op(data[i * 2], data[i * 2 + 1]);
        }
        SegTree {
            n,
            e,
            op,
            size,
            data,
            lazy: vec![id; size * 2],
            mapping,
            composition,
            id,
        }
    }

    pub fn eval(&mut self, k: usize) {
        if self.lazy[k] == self.id {
            return;
        }
        if k < self.size {
            self.lazy[k * 2] = (self.composition)(self.lazy[k], self.lazy[k * 2]);
            self.lazy[k * 2 + 1] = (self.composition)(self.lazy[k], self.lazy[k * 2 + 1]);
        }
        self.data[k] = (self.mapping)(self.lazy[k], self.data[k]);
        self.lazy[k] = self.id;
    }
    fn prod_rec(&mut self, tl: usize, tr: usize, ck: usize, cl: usize, cr: usize) -> S {
        // t: target, c: current
        self.eval(ck);
        if tr <= cl || cr <= tl {
            return self.e;
        }
        if tl <= cl && cr <= tr {
            return self.data[ck];
        } else {
            let half = (cl + cr) / 2;
            let lv = self.prod_rec(tl, tr, ck * 2, cl, half);
            let rv = self.prod_rec(tl, tr, ck * 2 + 1, half, cr);
            return (self.op)(lv, rv);
        }
    }
    pub fn prod(&mut self, l: usize, r: usize) -> S {
        self.prod_rec(l, r, 1, 0, self.size) // 1-indexed
    }

    fn apply_rec(&mut self, tl: usize, tr: usize, f: F, ck: usize, cl: usize, cr: usize) {
        self.eval(ck);
        if tr <= cl || cr <= tl {
            return;
        }
        if tl <= cl && cr <= tr {
            self.lazy[ck] = (self.composition)(f, self.lazy[ck]);
            self.eval(ck);
        } else {
            let half = (cl + cr) / 2;
            self.apply_rec(tl, tr, f, ck * 2, cl, half);
            self.apply_rec(tl, tr, f, ck * 2 + 1, half, cr);
            self.data[ck] = (self.op)(self.data[ck * 2], self.data[ck * 2 + 1]);
        }
    }
    pub fn apply(&mut self, l: usize, r: usize, f: F) {
        self.apply_rec(l, r, f, 1, 0, self.size);
    }
    pub fn print_vals(&self) {
        println!(
            "{:?}",
            self.data[self.size..self.size + self.n]
                .iter()
                .collect::<Vec<_>>()
        );
    }
}

// https://atcoder.jp/contests/arc045/tasks/arc045_b
#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        stl: [(Usize1, Usize1); m],
    }
    let op = |a: i64, b| a.min(b);
    let e = 1e9 as i64;
    let id: i64 = 0;
    let mapping = |f, x| f + x;
    let composition = |f, g| f + g;
    let mut seg = SegTree::new_from_vec(&vec![0; n], e, op, id, mapping, composition);

    for &(s, t) in &stl {
        seg.apply(s, t + 1, 1);
    }

    let mut ans = vec![];
    for i in 0..m {
        let (s, t) = stl[i];
        seg.apply(s, t + 1, -1);
        if seg.prod(0, n) > 0 {
            ans.push(i + 1);
        }
        seg.apply(s, t + 1, 1);
    }

    println!("{}", ans.len());
    for &a in &ans {
        println!("{}", a);
    }
}
