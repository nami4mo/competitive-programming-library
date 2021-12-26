#![allow(unused_imports, dead_code, unused_macros)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::VecDeque;

// ax + by = 1
fn _ext_gcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let q = a / b;
    let r = a % b;
    /*
        (qb+r)x + by = 1
        => b(qx+y) + rx = 1
        => bs + rt = 1 (s.t.) s=qx+y, t=x
    */
    let (s, t) = _ext_gcd(b, r);
    let x = t;
    let y = s - q * t;
    (x, y)
}

/*
    ax + by = c -> (x0, y0, xd, yd)
    @ x = x0 + xd*t
    @ y = y0 + yd*t
    @ x0 is minimum non-negative integer
      (or "x0 < 0" && "xd == 0"  (e.g.) 3x = -15)
    NOTICE: Do not input (a,b,c) = (0,0,0)
*/
pub fn ext_gcd(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)> {
    if a == 0 && b == 0 {
        if c == 0 {
            return Some((0, 0, 0, 0)); // exception ( any (x,y) is OK )
        }
        return None;
    }
    let d = num::integer::gcd(a, b);
    if c % d != 0 {
        return None;
    }
    let (a, b, c) = (a / d, b / d, c / d);
    let (mut x, mut y) = _ext_gcd(a, b);
    x *= c;
    y *= c;
    if a as i128 * x as i128 + b as i128 * y as i128 == c as i128 * (-1) {
        x *= -1;
        y *= -1;
    }
    let (mut xd, mut yd) = (b, -a);
    if xd == 0 {
        return Some((x, y, xd, yd));
    }
    if xd < 0 {
        xd *= -1;
        yd *= -1;
    }
    /* calc min x (>= 0) */
    if x >= 0 {
        let minus_cnt = x / xd;
        x -= minus_cnt * xd;
        y -= minus_cnt * yd;
    } else if x < 0 {
        let plus_cnt = (-x - 1) / xd + 1;
        x += plus_cnt * xd;
        y += plus_cnt * yd;
    }
    Some((x, y, xd, yd))
}

/*
    solve "ax = 1 (mod m)"
    NOTICE: if m == 1 -> x = 0
*/
pub fn modinv(a: u64, m: u64) -> Option<u64> {
    if num::integer::gcd(a, m) != 1 || m == 0 {
        return None;
    }
    let m = m as i64;
    let (mut x, _y) = _ext_gcd(a as i64, m);
    x = ((x % m) + m) % m;
    Some(x as u64)
}

/*
    solve "ax = b (mod m)"
    NOTICE: if m == 1 -> x = 0
*/
pub fn ax_b_mod_m(a: u64, mut b: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    if m == 1 {
        return Some(0); // all x == all y (mod 1)
    }
    b %= m;
    if a == 0 {
        if b == 0 {
            return Some(0);
        }
        return None;
    }
    let d = num::integer::gcd(a, m);
    if d != 1 {
        if b % d != 0 {
            return None;
        }
    }
    let (a, b, m) = (a / d, b / d, m / d);
    let minv = modinv(a, m);
    match minv {
        None => None, // assert do not reach here. (modinv always exsits.)
        Some(v) => Some((b * v) % m),
    }
}

// https://atcoder.jp/contests/abc186/tasks/abc186_e
// #[fastout]
fn main() {
    input! {t: usize}
    for _ in 0..t {
        input! {n: i64, s: i64, k: i64}
        let v = ext_gcd(k, -n, -s);
        match v {
            None => println!("-1"),
            Some(v) => {
                let (x, _y, _xd, _yd) = v;
                println!("{}", x);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extgcd_by_bruteforce() {
        let nmax = 20;
        for a in -nmax..nmax {
            for b in -nmax..nmax {
                for c in -nmax..nmax {
                    match ext_gcd(a, b, c) {
                        None => {
                            for x in -nmax..nmax {
                                for y in -nmax..nmax {
                                    assert!(a * x + b * y != c);
                                }
                            }
                        }
                        Some(v) => {
                            let (x, y, xd, yd) = v;
                            assert!(x >= 0 || xd == 0);
                            assert!(x - xd < 0 || xd == 0);
                            for t in -3..3 {
                                let val = a * (x + xd * t) + b * (y + yd * t);
                                assert!(val == c, "{:?} -> {:?}", (a, b, c), v);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_modinv() {
        for a in 0..100 {
            for m in 0..100 {
                let minv = modinv(a, m);
                match minv {
                    None => {
                        for i in 1..m {
                            let val = (a * i) % m;
                            assert!(val != 1);
                        }
                    }
                    Some(v) => {
                        assert!(v < m);
                        assert!((v * a) % m == 1 % m, "a: {}, m: {}, minv: {}", a, m, v);
                    }
                }
            }
        }
    }

    #[test]
    fn test_ax_b_mod_m() {
        let nmax = 50;
        for a in 0..nmax {
            for m in 0..nmax {
                for b in 0..nmax {
                    let x = ax_b_mod_m(a, b, m);
                    match x {
                        None => {
                            for i in 1..m {
                                let val = (a * i) % m;
                                assert!(val != b, "a: {}, x: {}, b: {}, m: {}", a, i, b, m);
                            }
                        }
                        Some(v) => {
                            assert!(v < m);
                            assert!(
                                (a * v) % m == b % m,
                                "a: {}, x: {}, b: {}, m: {}",
                                a,
                                v,
                                b,
                                m
                            );
                        }
                    }
                }
            }
        }
    }
}
