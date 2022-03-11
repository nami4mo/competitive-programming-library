use bitset_fixed::BitSet;
use proconio::{input, marker::Usize1};

/*
    掃き出し法（xor）
    bit_rev == true -> bitの大きい方から確定
    bit_rev == false -> bitの小さい方から確定

    mat を変形し、rank を return
*/
pub fn gauss_jordan_f2(mat: &mut Vec<BitSet>, bit_rev: bool) -> usize {
    let mut rank = 0;
    let max_bit = mat[0].size(); // col
    let row_n = mat.len();
    for i in 0..max_bit {
        let bit_i = if bit_rev { max_bit - i - 1 } else { i };
        let mut next_row = None;
        for row in rank..row_n {
            if mat[row][bit_i] {
                next_row = Some(row);
                break;
            }
        }
        match next_row {
            None => continue,
            Some(next_row) => {
                mat.swap(next_row, rank);
                for row in 0..row_n {
                    let rank_row = mat[rank].clone();
                    if row != rank && mat[row][bit_i] {
                        mat[row] ^= &rank_row;
                    }
                }
                rank += 1;
            }
        }
    }
    rank
}

// https://atcoder.jp/contests/typical90/tasks/typical90_be
fn main() {
    input! {n: usize, m: usize}
    let mut mat = vec![BitSet::new(m); n];
    for i in 0..n {
        input! {
            t: usize,
            al: [Usize1; t],
        }
        for &a in &al {
            mat[i].set(a, true);
        }
    }
    input! {sl: [usize; m]}
    let mut target = BitSet::new(m);
    for i in 0..m {
        if sl[i] == 1 {
            target.set(i, true);
        }
    }

    let rank = gauss_jordan_f2(&mut mat, false);
    let mut panels = BitSet::new(m);

    for row in &mat {
        let mut lsb = None;
        for i in 0..m {
            if row[i] {
                lsb = Some(i);
                break;
            }
        }
        match lsb {
            None => (),
            Some(lsb) => {
                if panels[lsb] != target[lsb] {
                    panels ^= &row;
                }
            }
        }
    }

    if (panels ^ &target).count_ones() == 0 {
        let mut ans = 1;
        let mo = 998244353;
        for _ in 0..(n - rank) {
            ans *= 2;
            ans %= mo;
        }
        println!("{}", ans);
    } else {
        println!("0");
    }
}
