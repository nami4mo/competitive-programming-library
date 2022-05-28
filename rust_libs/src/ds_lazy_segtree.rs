#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// @doc.begin [Rust/lazy_segtree] {LazySegTree}
// @doc.src_c.begin
pub struct LazySegTree<S, Op, F, Mapping, Composition>
where
    S: Copy + std::fmt::Debug,
    Op: Fn(S, S) -> S,
    F: Copy + std::cmp::PartialEq + std::fmt::Debug,
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

impl<S, Op, F, Mapping, Composition> LazySegTree<S, Op, F, Mapping, Composition>
where
    S: Copy + std::fmt::Debug,
    Op: Fn(S, S) -> S,
    F: Copy + std::cmp::PartialEq + std::fmt::Debug,
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
        LazySegTree {
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
    pub fn print_vals(&mut self) {
        // update leaf vals
        for i in 0..self.n {
            self.prod(i, i + 1);
        }
        println!(
            "{:?}",
            self.data[self.size..self.size + self.n]
                .iter()
                .collect::<Vec<_>>()
        );
    }
}
// @doc.src_c.end
/* @doc.text.begin
## 使い方

### 初期化

```rs
let mut seg = LazySegTree::new_from_vec(&vals, e, op, id, mapping, composition);
```

- `vals`: 初期の `Vec`
- `e`: 二項演算の単位元（例: 区間加算なら `0`、区間 min なら `INF`）
- `op`: 二項演算（例: 区間加算なら `|a,b| a+b`）
- `id`: マッピングの恒等写像（区間加算更新なら `0`、区間変更ならあり得ない値 `INF`）
- `mapping`: 区間操作演算
- `composition`: 写像の合成（`|f,g| f(g(x))`）


### 更新
```rs
seg.apply(left, right, f)  # [left, right)
```

### 値取得
```rs
seg.prod(left, right)  # [left, right)
```

@doc.text.end */

// @doc.subtitle {例題}
// @doc.text.inline [ARC045-B](https://atcoder.jp/contests/arc045/tasks/arc045_b): 区間加算更新 & 区間最小値取得
// @doc.src.begin
fn solve() {
    input! {
        n: usize, m: usize,
        stl: [(Usize1, Usize1); m],
    }
    let op = |a: i64, b| a.min(b);
    let e = 1e9 as i64;
    let id: i64 = 0;
    let mapping = |f, x| f + x;
    let composition = |f, g| f + g;
    let mut seg = LazySegTree::new_from_vec(&vec![0; n], e, op, id, mapping, composition);

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
// @doc.src.end

// @doc.subtitle {例題}
// @doc.text.inline 区間更新 & 区間加算取得
// @doc.src.begin
fn solve2() {
    type P = (i64, i64);
    let vals = [1, 2, 3, 4, 5];
    let vals = vals.iter().map(|&v| (v, 1)).collect_vec();
    let op = |a: P, b: P| (a.0 + b.0, a.1 + b.1);
    let e = (0, 0);
    let id = std::i64::MAX;
    let mapping = |f: i64, x: P| {
        if f == id {
            x
        } else {
            (f * x.1, x.1)
        }
    };
    let composition = |f: i64, g: i64| {
        if f == id {
            g
        } else {
            f
        }
    };
    let mut seg = LazySegTree::new_from_vec(&vals, e, op, id, mapping, composition);
    assert!(seg.prod(0, 5).0 == 15);
    seg.apply(1, 3, 10);
    assert!(seg.prod(0, 3).0 == 21);
    seg.apply(2, 4, -5);
    assert!(seg.prod(0, 5).0 == 6);
    seg.apply(0, 1, -100);
    assert!(seg.prod(0, 5).0 == -95);
}
// @doc.src.end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lst_test_1() {
        type P = (i64, i64);

        let vals = [1, 2, 3, 4, 5];
        let vals = vals.iter().map(|&v| (v, 1)).collect_vec();
        let op = |a: P, b: P| (a.0 + b.0, a.1 + b.1);
        let e = (0, 0);
        let id = std::i64::MAX;
        let mapping = |f: i64, x: P| {
            if f == id {
                x
            } else {
                (f * x.1, x.1)
            }
        };
        let composition = |f: i64, g: i64| {
            if f == id {
                g
            } else {
                f
            }
        };
        let mut seg = LazySegTree::new_from_vec(&vals, e, op, id, mapping, composition);
        assert!(seg.prod(0, 5).0 == 15);
        seg.apply(1, 3, 10);
        assert!(seg.prod(0, 3).0 == 21);
        seg.apply(2, 4, -5);
        assert!(seg.prod(0, 5).0 == 6);
        seg.apply(0, 1, -100);
        assert!(seg.prod(0, 5).0 == -95);
    }
}
