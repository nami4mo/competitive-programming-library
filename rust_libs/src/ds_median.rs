#![allow(unused_imports, dead_code, unused_macros)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::BTreeSet;

// @doc.begin [Rust/median] {Median}
// @doc.src_c.begin
struct Med<T: Copy + std::fmt::Debug + Ord> {
    l_st: BTreeSet<T>, // 左半分（中央値以下）
    r_st: BTreeSet<T>, // 右半分（中央値以上）
}

/*
    常に
    (A) l_st.len() - 1 == r_st.len()
    (B) l_st.len() == r_st.len()
    のどちらかの状態になる

    (A) のときは要素が奇数個で
    l_st.iter().last() が唯一の中央値

    (B) のときは要素が偶数個で
    l_st.iter().last() と r_st.iter().next() が中央値（の候補）
    問題によって どちらかが中央値になる or 平均をとる などが変わるので注意
*/

impl<T: Copy + std::fmt::Debug + Ord> Med<T> {
    pub fn new() -> Self {
        Med {
            l_st: BTreeSet::new(),
            r_st: BTreeSet::new(),
        }
    }
    pub fn insert(&mut self, x: T) {
        if self.l_st.is_empty() {
            self.l_st.insert(x);
        } else {
            if x < *self.l_st.iter().last().unwrap() {
                self.l_st.insert(x);
            } else {
                self.r_st.insert(x);
            }
        }
        self.adjust();
    }

    pub fn remove(&mut self, x: T) {
        // assert!(self.l_st.contains(&x) || self.r_st.contains(&x));
        self.l_st.remove(&x);
        self.r_st.remove(&x);
        self.adjust();
    }

    fn adjust(&mut self) {
        if self.l_st.len() < self.r_st.len() {
            let poped = *self.r_st.iter().next().unwrap();
            self.r_st.remove(&poped);
            self.l_st.insert(poped);
        } else if self.l_st.len() > self.r_st.len() + 1 {
            let poped = *self.l_st.iter().last().unwrap();
            self.l_st.remove(&poped);
            self.r_st.insert(poped);
        }
    }

    pub fn get_med(&self) -> (T, T) {
        if self.l_st.len() > self.r_st.len() {
            assert_eq!(self.l_st.len() - 1, self.r_st.len());
            // change this if necessary
            let x = *self.l_st.iter().last().unwrap();
            (x, x)
        } else {
            assert_eq!(self.l_st.len(), self.r_st.len());
            // change this if necessary
            (
                *self.l_st.iter().last().unwrap(),
                *self.r_st.iter().next().unwrap(),
            )
        }
    }
}
// @doc.src_c.end

/* @doc.text.begin
挿入/削除しながら中央値を管理するデータ構造。

## 使い方
- `Med<T: Copy + std::fmt::Debug + Ord>`
- **多重集合ではないので、同じ値を扱うときは `index` 込みの `tuple` にするなど工夫する**
- 中央値の定義に応じて `get_med()` の処理を変えるとよい

### 初期化

```rs
let mut med = Med::new();
```

### 追加
```rs
med.insert(x: T)
```

### 削除
存在しなくてもエラーにはならない。
```rs
med.remove(x: T)
```

### 中央値取得
```rs
let (v1, v2) = med.get_med();
```

- 要素数が奇数のとき: v1 == v2（中央の要素）
- 要素数が偶数のとき: v1 != v2（中央の左右の要素）

@doc.text.end */

// @doc.subtitle {例題}
// @doc.text.inline [ABC218-G](https://atcoder.jp/contests/abc185/tasks/abc218_g): 木をDFSで辿りながら集合の中央値を管理する
// @doc.src.begin
const INF: u64 = !0;
fn main() {
    input! {
        n: usize,
        al: [u64; n],
        es: [(Usize1, Usize1); n-1],
    }
    let mut gl = vec![vec![]; n];
    for &(u, v) in &es {
        gl[u].push(v);
        gl[v].push(u);
    }

    let mut med = Med::<(u64, usize)>::new();
    fn dfs(
        node: usize,
        pare: usize,
        gl: &Vec<Vec<usize>>,
        med: &mut Med<(u64, usize)>,
        al: &Vec<u64>,
        depth: usize,
    ) -> u64 {
        let mut mi = INF;
        let mut ma = 0;
        med.insert((al[node], node));
        for &neib in &gl[node] {
            if neib == pare {
                continue;
            }
            let val = dfs(neib, node, gl, med, al, depth + 1);
            mi = mi.min(val);
            ma = ma.max(val);
        }
        if mi == INF {
            let med_vals = med.get_med();
            med.remove((al[node], node));
            return (med_vals.0 .0 + med_vals.1 .0) / 2;
        }
        med.remove((al[node], node));
        if depth % 2 == 0 {
            ma
        } else {
            mi
        }
    }
    let ans = dfs(0, !0, &gl, &mut med, &al, 0);
    println!("{}", ans);
}
// @doc.src.end
