#![allow(unused_imports, dead_code, unused_macros)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// @doc.begin [Rust/bisect] {Bisect}
// @doc.src_c.begin
trait Bisect {
    type Item: Ord + Copy;
    fn bisect_left(&self, x: Self::Item) -> usize;
    fn bisect_right(&self, x: Self::Item) -> usize;

    /* --- count --- */
    fn less_eq_cnt(&self, x: Self::Item) -> usize; //     <= x
    fn less_cnt(&self, x: Self::Item) -> usize; //        < x
    fn greater_eq_cnt(&self, x: Self::Item) -> usize; //  >= x
    fn greater_cnt(&self, x: Self::Item) -> usize; //     > x

    /* --- nearest value --- */
    fn less_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
    fn less_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
    fn greater_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
    fn greater_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
}

impl<T: Ord + Copy> Bisect for Vec<T> {
    type Item = T;
    fn bisect_left(&self, x: Self::Item) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let mid = (ng + ok) / 2;
            if x <= self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    fn bisect_right(&self, x: Self::Item) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i64;
        while ok - ng > 1 {
            let mid = (ng + ok) / 2;
            if x < self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    fn less_eq_cnt(&self, x: Self::Item) -> usize {
        self.bisect_right(x)
    }
    fn less_cnt(&self, x: Self::Item) -> usize {
        self.bisect_left(x)
    }
    fn greater_eq_cnt(&self, x: Self::Item) -> usize {
        self.len() - self.bisect_left(x)
    }
    fn greater_cnt(&self, x: Self::Item) -> usize {
        self.len() - self.bisect_right(x)
    }
    fn less_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)> {
        let ind = self.bisect_right(x);
        match ind {
            0 => None,
            _ => Some((ind - 1, self[ind - 1])),
        }
    }
    fn less_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)> {
        let ind = self.bisect_left(x);
        match ind {
            0 => None,
            _ => Some((ind - 1, self[ind - 1])),
        }
    }
    fn greater_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)> {
        let ind = self.bisect_left(x);
        if ind == self.len() {
            None
        } else {
            Some((ind, self[ind]))
        }
    }
    fn greater_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)> {
        let ind = self.bisect_right(x);
        if ind == self.len() {
            None
        } else {
            Some((ind, self[ind]))
        }
    }
}
// @doc.src_c.end

/* @doc.text.begin
## 使い方
ライブラリをコピペすれば、`Vec<T: Ord+Copy>` に `Bisect` が実装される。

### 以下/未満/以上/より大きい 個数を数える

```rs
fn less_eq_cnt(&self, x: Self::Item) -> usize; //     <= x
fn less_cnt(&self, x: Self::Item) -> usize; //        < x
fn greater_eq_cnt(&self, x: Self::Item) -> usize; //  >= x
fn greater_cnt(&self, x: Self::Item) -> usize; //     > x

// 例: 10 以上の値の個数
let cnt = vec.greater_eq_cnt(10);
```

### 以下/未満/以上/より大きい で 最も近い値（index, 値）を取得する
```rs
fn less_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
fn less_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
fn greater_eq_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;
fn greater_nearest(&self, x: Self::Item) -> Option<(usize, Self::Item)>;

// 例: 10より大きい最も近い値（index, 値）
let res = vec.greater_nearest(10);
match res{
    None => (),
    Some((ind,val)) => {
        //
    }
}
```


@doc.text.end */

// #[fastout]
fn main() {
    // https://atcoder.jp/contests/typical90/tasks/typical90_g
    input!(n: usize, mut al: [i64; n], q: usize);
    al.sort();
    for _ in 0..q {
        input! {b: i64}
        let res1 = al.less_eq_nearest(b);
        let res2 = al.greater_eq_nearest(b);
        let mut ans = 10i64.pow(18);
        match res1 {
            Some(v) => ans = ans.min(num::abs(b - v.1)),
            _ => (),
        }
        match res2 {
            Some(v) => ans = ans.min(num::abs(b - v.1)),
            _ => (),
        }
        println!("{}", ans);
    }

    //// https://atcoder.jp/contests/abc077/tasks/arc084_a
    // input! {
    //     n: usize,
    //     mut al: [i64; n],
    //     mut bl: [i64; n],
    //     mut cl: [i64; n],
    // }
    // al.sort();
    // bl.sort();
    // cl.sort();
    // let mut ans = 0;
    // for &b in &bl {
    //     ans += al.less_cnt(b) * cl.greater_cnt(b);
    // }
    // println!("{}", ans);
}
