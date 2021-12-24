#![allow(unused_imports)]
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// ただの累積和でごまかした簡潔ビットベクトル
// そのうちちゃんと作る
pub struct SuccinctBitVectorDummy {
    size: usize,
    bits: Vec<u8>,
    cumsum: Vec<u32>,
}

impl SuccinctBitVectorDummy {
    pub fn new(size: usize) -> Self {
        SuccinctBitVectorDummy {
            size,
            bits: vec![0; size],
            cumsum: vec![0; size + 1],
        }
    }
    pub fn set(&mut self, ind: usize, bit: u8) {
        self.bits[ind] = bit;
    }
    pub fn build(&mut self) {
        let mut c_sum = 0;
        for i in 0..self.size {
            if self.bits[i] > 0 {
                c_sum += 1;
            }
            self.cumsum[i + 1] = c_sum;
        }
    }
    pub fn access(&self, ind: usize) -> u8 {
        self.bits[ind]
    }
    pub fn rank(&self, ind: usize) -> u32 {
        // [0, ind)
        self.cumsum[ind]
    }
    pub fn select(&self, x: u32) -> usize {
        // x番目の1
        assert!(x > 0, "rank must be >0");
        let mut ng = 0;
        let mut ok = self.size + 1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self.cumsum[mid] < x {
                ng = mid;
            } else {
                ok = mid;
            }
        }
        // if ok == self.size -> no such ind
        ok - 1
    }
}

// https://atcoder.jp/contests/abc084/tasks/abc084_d
// #[fastout]
fn main() {
    input! {q: usize}
    let nmax = 1e5 as usize + 10;
    let nmax = 15;
    let mut primes = vec![true; nmax];
    primes[1] = false;
    primes[0] = false;
    for i in 1..nmax {
        let sq = i.sqrt();
        for j in 2..sq + 2 {
            if i % j == 0 && i != j {
                primes[i] = false;
                break;
            }
        }
    }
    let mut sbv = SuccinctBitVectorDummy::new(nmax);
    for i in 1..nmax {
        if i % 2 == 0 {
            continue;
        }
        if primes[i] && primes[(i + 1) / 2] {
            sbv.set(i, 1);
        }
    }
    sbv.build();

    for _ in 0..q {
        input! {l: usize, r:usize};
        let ans = sbv.rank(r + 1) - sbv.rank(l);
        println!("{}", ans);
    }
}
