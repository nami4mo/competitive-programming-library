#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub struct MultiSet<T>
where
    T: Eq + std::hash::Hash + std::cmp::Ord + Copy,
{
    pub st: BTreeSet<(T, usize)>,
    cnts: HashMap<T, usize>,
}

impl<T> MultiSet<T>
where
    T: Eq + std::hash::Hash + std::cmp::Ord + Copy,
{
    const CNT_MAX: usize = 1000000000;

    pub fn new() -> Self {
        Self {
            st: BTreeSet::new(),
            cnts: HashMap::new(),
        }
    }

    pub fn from_vec(vl: &Vec<T>) -> Self {
        let mut res = Self::new();
        for &v in vl {
            res.insert(v);
        }
        res
    }

    pub fn insert(&mut self, x: T) {
        *self.cnts.entry(x).or_insert(0) += 1;
        self.st.insert((x, self.cnts[&x] - 1));
    }

    pub fn remove(&mut self, x: T) {
        if *self.cnts.entry(x).or_insert(0) == 0 {
            return;
        }
        self.st.remove(&(x, self.cnts[&x] - 1));
        *self.cnts.get_mut(&x).unwrap() -= 1;
    }

    pub fn nearest_less_eq(&mut self, x: T) -> Option<T> {
        self.st.range(..=(x, Self::CNT_MAX)).last().map(|v| v.0)
    }

    pub fn nearest_less(&mut self, x: T) -> Option<T> {
        self.st.range(..(x, 0)).last().map(|v| v.0)
    }

    pub fn nearest_greater_eq(&mut self, x: T) -> Option<T> {
        self.st.range((x, 0)..).next().map(|v| v.0)
    }

    pub fn nearest_greater(&mut self, x: T) -> Option<T> {
        self.st.range((x, Self::CNT_MAX)..).next().map(|v| v.0)
    }
}

// https://atcoder.jp/contests/abc245/tasks/abc245_e
fn main() {
    input! {
        n: usize, m: usize,
        al: [usize; n],
        bl: [usize; n],
        cl: [usize; m],
        dl: [usize; m],
    }

    let abl = al
        .iter()
        .zip(bl.iter())
        .map(|(&a, &b)| (a, b))
        .sorted_by_key(|v| Reverse(*v))
        .collect::<Vec<_>>();
    let mut cdl = cl
        .iter()
        .zip(dl.iter())
        .map(|(&c, &d)| (c, d))
        .sorted_by_key(|v| Reverse(*v))
        .collect::<VecDeque<_>>();

    let mut mst = MultiSet::new();
    for &(a, b) in &abl {
        while !cdl.is_empty() && cdl.front().unwrap().0 >= a {
            let poped = cdl.pop_front().unwrap();
            mst.insert(poped.1);
        }
        let v = mst.nearest_greater_eq(b);
        match v {
            None => {
                println!("No");
                return;
            }
            Some(v) => mst.remove(v),
        }
    }
    println!("Yes");
}
