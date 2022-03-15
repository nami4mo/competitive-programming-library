#![allow(unused_imports)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// fn osa_k(vl: &Vec<usize>) -> Vec<HashMap<usize, usize>> {
fn osa_k(vl: &Vec<usize>) -> Vec<Vec<(usize, usize)>> {
    if vl.is_empty() {
        return vec![];
    }
    let vmax = *vl.iter().max().unwrap();
    let mut min_primes = (0..vmax + 1).map(|i| i).collect_vec();
    for prime in 2..vmax.sqrt() + 1 {
        if min_primes[prime] != prime {
            continue; // not prime
        }
        let mut curr = prime;
        while curr <= vmax {
            min_primes[curr] = min_primes[curr].min(prime);
            curr += prime;
        }
    }
    vl.iter()
        .map(|&v| {
            let mut pf_cnt = HashMap::new();
            let mut curr = v;
            while curr > 1 {
                let min_p = min_primes[curr];
                *pf_cnt.entry(min_p).or_insert(0) += 1;
                curr /= min_p;
            }
            pf_cnt.iter().map(|&k| k).collect() // into tuple
        })
        .collect()
}

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
