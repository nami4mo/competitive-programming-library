#![allow(unused_imports)]
use im_rc::HashMap;
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// @doc.begin [Rust/Math/divisor] {Divisor}
// @doc.src_c.begin
// Vec<HashMap> で返したい場合は、こちらの宣言に変える
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

fn prime_fac_perm(n: usize, k: usize) -> HashMap<usize, usize> {
    let mut p_facs = HashMap::new();
    let start = n - k + 1;
    let half = k.max(n.sqrt() + 1);
    let mut primes = vec![true; half + 1];
    let mut nums = (0..k).map(|i| start + i).collect_vec();
    for prime in 2..half + 1 {
        if !primes[prime] {
            continue;
        }
        let mut curr = prime * 2;
        while curr <= half {
            primes[curr] = false;
            curr += prime;
        }

        let offset = (prime - start % prime) % prime;
        let mut curr = start + offset;
        let mut cnt = 0;
        while curr <= n {
            while nums[curr - start] % prime == 0 {
                nums[curr - start] /= prime;
                cnt += 1;
            }
            curr += prime;
        }
        if cnt > 0 {
            p_facs.insert(prime, cnt);
            // *p_facs.entry(prime).or_insert(0) += cnt;
        }
    }
    for i in 0..k {
        if nums[i] != 1 {
            *p_facs.entry(nums[i]).or_insert(0) += 1;
        }
    }
    p_facs
}

/* 約数列挙 */
fn divs(n: usize) -> Vec<usize> {
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
fn prime_fac(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    if n == 1 {
        return res;
    }
    let sq = n.sqrt();
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

// オイラーのφ関数
// n 以下の n と互いに素な正整数の個数を数える
fn euler_totient(n: usize) -> usize {
    let pf = prime_fac(n);
    let mut ans = n;
    for &(p, _) in &pf {
        ans *= p - 1;
        ans /= p;
    }
    ans as usize
}
// @doc.src_c.end

/* @doc.text.begin
## osa_k 法
配列の高速な素因数分解
```rs
fn osa_k(vl: &Vec<usize>) -> Vec<Vec<(usize, usize)>>
let res = osa_k(&vec![12,30,5]);
# [ [(2,2),(3,1)], [(2,1),(3,1),(5,1)], [(5,1)] ]
```

## nPr の素因数分解（n < 10^12, k < 10^6 程度）
```rs
fn prime_fac_perm(n: usize, k: usize) -> HashMap<usize, usize>
```

## オイラーのφ関数
n 以下の n と互いに素な正整数の個数を数える
```rs
fn euler_totient(n: usize) -> usize
```

## 素因数分解
```rs
fn prime_fac(mut n: usize) -> Vec<(usize, usize)> {
```

## 約数列挙
```rs
fn divs(n: usize) -> Vec<usize>
```

@doc.text.end */

#[cfg(test)]
mod test {
    use itertools::zip;

    use super::*;

    #[test]
    fn test_euler_totient() {
        let ans = vec![
            0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20,
            12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46,
            16, 42, 20,
        ];
        for i in 1..51 {
            assert_eq!(euler_totient(i), ans[i]);
        }
    }

    #[test]
    fn test_osa_k() {
        let vals = vec![24, 100, 45, 8, 97, 105, 10000, 1, 12342];
        let osa_k_res = osa_k(&vals);
        for (ans, res) in zip(vals, osa_k_res) {
            let mut val = 1;
            for &(p, c) in &res {
                val *= p.pow(c as u32);
            }
            assert_eq!(ans, val);
        }
    }
}
