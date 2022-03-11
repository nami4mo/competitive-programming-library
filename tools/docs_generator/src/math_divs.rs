#![allow(unused_imports)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

/* 約数列挙 */
fn divs(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let sq = num_integer::sqrt(n);
    for i in 1..sq + 1 {
        if n % i == 0 {
            res.push(i);
            if i * i != n {
                res.push(n / i);
            }
        }
    }
    res.sort();
    res
}

/* 素因数分解 */
fn prime_fac(mut n: u64) -> Vec<(u64, usize)> {
    let mut res = vec![];
    if n == 1 {
        return res;
    }
    let sq = num_integer::sqrt(n);
    for i in 2..sq + 1 {
        if n % i == 0 {
            let mut cnt = 0;
            while n % i == 0 {
                cnt += 1;
                n /= i;
            }
            res.push((i, cnt));
        }
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

// #[fastout]
fn main() {
    input! {n: u64}
    let pfs = prime_fac(n);
    let cnt: usize = pfs.iter().map(|x| x.1).sum();
    let mut val = 1;
    let mut ans = 0;
    loop {
        if cnt <= val {
            println!("{}", ans);
            break;
        }
        ans += 1;
        val *= 2;
    }
}
