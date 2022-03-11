#![allow(unused_imports, dead_code, unused_macros)]
use num_integer::gcd;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::collections::VecDeque;

// @doc.begin [Rust/Math/integer] {Integer}
// @doc.src_c.begin
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
        None => panic!("something wrong. modinv always exists."),
        Some(v) => Some((b * v) % m),
    }
}
// @doc.src_c.end

/* @doc.text.begin
## 使い方
注) `a = 0` などの例外的なケースは別途処理したほうが無難
注) Rust の mod演算（`%`） は、左辺が負のときは負を返すので注意


### ext_gcd ( ax + by = c )
```rs
fn ext_gcd(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)>
```

- 解なしのときは `None`
- 解ありのときは `Some(x0, y0, xd, yd)`
    - `x = x0 + xd*t`, `y = y0 + yd*t`
    - `x0` は最小の 非負整数
    - ただし、`x` が 負の値しか取らない時はその値を返す（3x + 0y = 15 など）

### ax_b_mod_m ( ax = b (mod m) )
```rs
fn ax_b_mod_m(a: u64, mut b: u64, m: u64) -> Option<u64>
```

- 解なしのときは `None`
- 解ありのときは `Some(x)  (x < m)`

### modinv ( a の逆元 (mod m) )
```rs
fn modinv(a: u64, m: u64) -> Option<u64>
```



@doc.text.end */

// @doc.subtitle {例題: ext_gcd}
// @doc.text.inline [ABC186-E](https://atcoder.jp/contests/abc186/tasks/abc186_e): kx + (-n)y = -s を満たす最小の非負整数 x を求める問題に帰着する
// @doc.src.begin
fn solve_ext_gcd() {
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
// @doc.src.end

// @doc.subtitle {例題: ax_b_mod_m}
// @doc.text.inline [ABC186-E](https://atcoder.jp/contests/abc186/tasks/abc186_e): ↑と同じ問題を kx = n - s (mod n)  で解く
// @doc.src.begin
fn solve_ax_b_mod_m() {
    input! {t: usize}
    for _ in 0..t {
        input! {n: u64, s: u64, k: u64}
        let v = ax_b_mod_m(k, n - s, n);
        match v {
            None => println!("-1"),
            Some(x) => {
                println!("{}", x);
            }
        }
    }
}
// @doc.src.end

// @doc.subtitle {例題: modinv}
// @doc.text.inline [ABC186-E](https://atcoder.jp/contests/abc186/tasks/abc186_e): ↑と同じ問題を kの逆元（mod n） を両辺にかけて解く
// @doc.src.begin
fn solve_modinv() {
    input! {t: usize}
    for _ in 0..t {
        input! {n: u64, s: u64, k: u64}
        // k と n が互いに素でないと逆元が存在しない
        let g = gcd(k, n);
        if (n - s) % g != 0 {
            println!("-1");
            continue;
        }
        // 以降、全てを k と n の GCD で割った値
        let kinv = modinv(k / g, n / g);
        match kinv {
            None => println!("-1"),
            Some(kinv) => {
                let ans = ((n - s) * kinv / g) % (n / g);
                println!("{}", ans);
            }
        }
    }
}
// @doc.src.end
