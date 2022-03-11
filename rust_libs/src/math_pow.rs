#![allow(unused_imports, dead_code, unused_macros)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// @doc.begin [Rust/Math/pow_mod] {PowMod}
// @doc.src_c.begin
fn pow_mod(base: u64, exp: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut curr_val = base;
    let mut exp_rem = exp;
    while exp_rem > 0 {
        if exp_rem & 1 == 1 {
            res *= curr_val;
            res %= m;
        }
        curr_val *= curr_val;
        curr_val %= m;
        exp_rem >>= 1;
    }
    res
}
macro_rules! pow_mod {
    ($base: expr, $exp: expr, $mo: expr) => {
        pow_mod($base as u64, $exp as u64, $mo as u64)
    };
}
// @doc.src_c.end

/* @doc.text.begin
## 使い方
```rs
let x = pow_mod!(2,1000);
```

- 型は `u64` にキャスト可能であれば OK

@doc.text.end */

fn main() {
    let n = 10usize;
    let _a = pow_mod!(n, n, 10i64.pow(9) + 7);
}
