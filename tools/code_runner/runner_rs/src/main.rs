#![allow(unused_imports)]
use libm::sqrt;
use num_integer::Roots;
use proconio::{fastout, input, marker::Chars, marker::Usize1};

fn l_to_i(x: &Vec<u64>) -> u64 {
    let mut res = 0;
    let mut ten = 1;
    for i in 0..x.len() {
        res += ten * x[x.len() - 1 - i];
        ten *= 10;
    }
    res
}

fn main() {
    input! {
        //
        xx: Chars,
    }
    let x = xx
        .iter()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    let xv = l_to_i(&x);
    let mut ans = 1e19 as u64;
    for neib in 0..2 {
        for d in -10..10 {
            let mut new_vec = vec![];
            let mut ok = true;
            for i in 0..x.len() {
                let v = x[0] as i64 + i as i64 * d - neib;
                if v >= 10 || v < 0 {
                    ok = false;
                    break;
                }
                new_vec.push(v as u64);
            }
            if ok {
                let val = l_to_i(&new_vec);
                if val >= xv {
                    ans = ans.min(val);
                }
            }
        }
    }
    println!("{}", ans);
}
