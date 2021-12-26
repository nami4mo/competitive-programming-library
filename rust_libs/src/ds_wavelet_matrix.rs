#![allow(unused_imports)]
use im_rc::{HashMap, HashSet};
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

// ただの累積和でごまかした簡潔ビットベクトル
// いつか作る
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
    pub fn rank1(&self, ind: usize) -> u32 {
        // [0, ind)
        self.cumsum[ind]
    }
    pub fn rank0(&self, ind: usize) -> u32 {
        ind as u32 - self.rank1(ind)
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
    pub fn print_bits(&self) {
        println!("{:?}", self.bits);
    }
}

// reference:
// https://github.com/MitI-7/WaveletMatrix/blob/master/WaveletMatrix/WaveletMatrix.hpp
pub struct WaveletMatrix {
    #[allow(dead_code)]
    size: usize,
    bit_len: usize,
    bit_arrays: Vec<SuccinctBitVectorDummy>,
    bit_arrays_zero_cnts: Vec<usize>,
    bit_last_begins: HashMap<u64, usize>,
    #[allow(dead_code)]
    bit_last_zero_cnt: usize,
    max_elem: u64,
}

impl WaveletMatrix {
    pub fn new(vals: &Vec<u64>) -> Self {
        let size = vals.len();
        let &max_elem = vals.iter().max().unwrap();
        let mut bit_val = 2;
        let mut bit_len = 1;
        while max_elem > bit_val - 1 {
            bit_val *= 2;
            bit_len += 1;
        }

        let mut bit_arrays = vec![];
        let mut bit_arrays_zero_cnts = vec![];
        let mut c_vals = vals.clone();
        for bit_i in (0..bit_len).rev() {
            let mut sbv = SuccinctBitVectorDummy::new(size);
            let mut zero_vals = vec![];
            let mut one_vals = vec![];
            for (i, &v) in c_vals.iter().enumerate() {
                if v & (1 << bit_i) > 0 {
                    sbv.set(i, 1);
                    one_vals.push(v);
                } else {
                    zero_vals.push(v);
                }
            }
            sbv.build();
            bit_arrays.push(sbv);
            bit_arrays_zero_cnts.push(zero_vals.len());
            c_vals = [zero_vals, one_vals].concat();
        }

        let mut bit_last_begins = HashMap::new();
        let bit_last_zero_cnt = bit_arrays_zero_cnts[bit_len - 1];
        for (i, &v) in c_vals.iter().enumerate() {
            bit_last_begins.entry(v).or_insert(i);
        }

        WaveletMatrix {
            size,
            bit_len,
            bit_arrays,
            bit_arrays_zero_cnts,
            bit_last_begins,
            bit_last_zero_cnt,
            max_elem,
        }
    }

    pub fn access(&self, ind: usize) -> u64 {
        let mut r_ind = ind;
        let mut c_val = 0;
        for i in 0..self.bit_len {
            let bit = self.bit_arrays[i].access(r_ind);
            if bit == 1 {
                r_ind =
                    self.bit_arrays_zero_cnts[i] + self.bit_arrays[i].rank1(r_ind + 1) as usize - 1;
                c_val |= 1 << (self.bit_len - i - 1);
            } else {
                r_ind = self.bit_arrays[i].rank0(r_ind + 1) as usize - 1;
            }
        }
        c_val
    }

    pub fn rank(&self, ind: usize, x: u64) -> usize {
        // # of x in [0, ind)
        self.rank_lr(0, ind, x)
    }

    pub fn rank_lr(&self, mut l: usize, mut r: usize, x: u64) -> usize {
        // # of x in [l, r)
        if !self.bit_last_begins.contains_key(&x) || l >= r {
            return 0;
        }
        for i in 0..self.bit_len {
            let bit = x & (1 << (self.bit_len - i - 1));
            if bit >= 1 {
                let l_rank1 = self.bit_arrays[i].rank1(l);
                let r_rank1 = self.bit_arrays[i].rank1(r);
                l = self.bit_arrays_zero_cnts[i] + l_rank1 as usize;
                r = self.bit_arrays_zero_cnts[i] + r_rank1 as usize;
            } else {
                let l_rank0 = self.bit_arrays[i].rank0(l);
                let r_rank0 = self.bit_arrays[i].rank0(r);
                l = l_rank0 as usize;
                r = r_rank0 as usize;
            }
        }
        r - l
    }

    pub fn rank_all(&self, mut l: usize, mut r: usize, x: u64) -> (usize, usize, usize) {
        if l >= r {
            return (0, 0, 0);
        }
        if self.max_elem < x {
            return (r - l, 0, 0);
        }
        let mut le_rank = 0;
        let mut gt_rank = 0;
        for i in 0..self.bit_len {
            let bit = x & (1 << (self.bit_len - i - 1));
            let curr_d = r - l;
            if bit >= 1 {
                let l_rank1 = self.bit_arrays[i].rank1(l);
                let r_rank1 = self.bit_arrays[i].rank1(r);
                l = self.bit_arrays_zero_cnts[i] + l_rank1 as usize;
                r = self.bit_arrays_zero_cnts[i] + r_rank1 as usize;
                le_rank += curr_d - (r - l);
            } else {
                let l_rank0 = self.bit_arrays[i].rank0(l);
                let r_rank0 = self.bit_arrays[i].rank0(r);
                l = l_rank0 as usize;
                r = r_rank0 as usize;
                gt_rank += curr_d - (r - l);
            }
        }
        (le_rank, r - l, gt_rank)
    }

    pub fn range_freq(&self, l: usize, r: usize, s: u64, t: u64) -> usize {
        if s >= t || l >= r {
            return 0;
        }
        let (less_s, _, _) = self.rank_all(l, r, s);
        let (less_t, _, _) = self.rank_all(l, r, t);
        less_t - less_s
    }
}

// https://atcoder.jp/contests/abc075/tasks/abc075_d
// #[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        tmp_xyl: [(i64,i64); n],
    }

    let offset = 1e9 as i64 + 10;
    let xyl = tmp_xyl
        .iter()
        .map(|v| ((v.0 + offset) as u64, (v.1 + offset) as u64))
        .collect::<Vec<_>>();

    let mut st = HashSet::new();
    for &(x, _) in &xyl {
        st.insert(x);
    }
    let mut xvals = st.into_iter().collect::<Vec<_>>();
    xvals.sort();
    let mut xcomp = HashMap::new();
    for i in 0..xvals.len() {
        xcomp.entry(xvals[i]).or_insert(i);
    }
    let mut ys = vec![0; xvals.len()];
    for &(x, y) in &xyl {
        let xx = xcomp[&x];
        ys[xx] = y;
    }
    let wm = WaveletMatrix::new(&ys);

    let mut ans = 1e19 as u64;
    for &x1 in &xvals {
        for &x2 in &xvals {
            if x1 >= x2 {
                continue;
            }
            for &y1 in &ys {
                for &y2 in &ys {
                    if y1 >= y2 {
                        continue;
                    }
                    let cnt = wm.range_freq(xcomp[&x1], xcomp[&x2] + 1, y1, y2 + 1);
                    if cnt >= k {
                        ans = ans.min((x2 - x1) * (y2 - y1));
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use crate::WaveletMatrix;

    #[test]
    fn wm_test() {
        let vals = vec![
            3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 9, 18, 20, 0, 999, 1, 2, 3, 4, 3,
        ];
        let wm = WaveletMatrix::new(&vals);

        for l in 0..vals.len() {
            for r in l + 1..vals.len() {
                for x in 0..10 {
                    let cnt_lr = wm.rank_lr(l, r, x);
                    let (cnt_le, cnt_eq, cnt_gt) = wm.rank_all(l, r, x);
                    let mut cnt2_le = 0;
                    let mut cnt2_eq = 0;
                    let mut cnt2_gt = 0;
                    for k in l..r {
                        if vals[k] < x {
                            cnt2_le += 1;
                        } else if vals[k] == x {
                            cnt2_eq += 1;
                        } else {
                            cnt2_gt += 1;
                        }
                    }
                    assert!(cnt_lr == cnt2_eq);
                    assert!(cnt_le == cnt2_le);
                    assert!(cnt_eq == cnt2_eq);
                    assert!(cnt_gt == cnt2_gt);
                }
            }
        }
    }
}
