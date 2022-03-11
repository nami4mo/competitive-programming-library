#![allow(unused_imports)]

use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

pub struct Comb {
    #[allow(dead_code)]
    inv: Vec<u64>,
    fac: Vec<u64>,
    finv: Vec<u64>,
    m: u64,
}

impl Comb {
    pub fn new(mut n: usize, m: u64) -> Self {
        n += 1;
        let mut fac = vec![0; n];
        let mut inv = vec![0; n];
        let mut finv = vec![0; n];
        for i in 0..2 {
            fac[i] = 1;
            inv[i] = 1;
            finv[i] = 1;
        }
        for i in 2..n {
            let iu64 = i as u64;
            fac[i] = (fac[i - 1] * iu64) % m;
            inv[i] = m - (inv[m as usize % i] * (m / iu64)) % m;
            finv[i] = (finv[i - 1] * inv[i]) % m;
        }
        Comb { fac, inv, finv, m }
    }
    pub fn com(&self, n: usize, r: usize) -> u64 {
        if n < r {
            return 0;
        }
        self.fac[n] * (self.finv[r] * self.finv[n - r] % self.m) % self.m
    }
    pub fn perm(&self, n: usize, r: usize) -> u64 {
        if n < r {
            return 0;
        }
        self.fac[n] * self.finv[n - r] % self.m
    }
    // TODO: lucas
    // def lucas(self, n, r): # nCr (mod self._mod(prime))
    //     if n < r: return 0
    //     res = 1
    //     while n > 0:
    //         nq, rq = n//self._mod, r//self._mod
    //         nr, rr = n-nq*self._mod, r-rq*self._mod
    //         res *= self.com(nr, rr)
    //         res %= self._mod
    //         n, r = nq, rq
    //     return res
}

// #[fastout]
fn main() {}
