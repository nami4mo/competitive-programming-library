#![allow(unused_imports, dead_code, unused_macros)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

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
