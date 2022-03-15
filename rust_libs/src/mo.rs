#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

/*
    # Mo's algorithm
    - クエリ先読み可能 && 値の更新なし の時に使用可能
    - 参考: https://aotamasaki.hatenablog.com/entry/2020/08/04/Mo%27s_algorithm_python
    - 抽象化が面倒だったので、フィールドや関数を直で書き換えて使用する

    ## 使い方
    [0] 処理したい列の型が Vec<usize> でないなら変更
    [1] 答えに必要な情報を保持するフィールドを用意（例: 値の出現回数など）
    [2] add(i): vals[i] の情報を追加する時の処理（例: vals[i] の出現回数+=1 など）
    [3] delete(i): vals[i] の情報を削除する時の処理（例: vals[i] の出現回数-=1 など）
    [4] 答えの型が Vec<i64> でないなら変更
    [5] 答え格納の Vec を初期化
    [6] 各クエリに応じた状態になっているので、ここで ans[qi] を更新
    [7] ans を return

    ## 例題
    https://atcoder.jp/contests/abc242/tasks/abc242_g
    区間 [l,r] 内にある同じ数同士でペアを作ると、最大いくつできる？
    各数の個数をカウントしながら、
    - ペアが作れるようになった（%2==0）ならペア数 += 1
    - ペアが作れなくなった（%2==1）ならペア数 -= 1
    クエリごとに、そのペア数を答えとして格納する
*/

struct Mo {
    // [1] add info for ans !!!!!!!
    // cnts: Vec<usize>,
    // pair_cnt: usize,

    //// [0] change vals type (if necessary) !!!!!!
    vals: Vec<usize>,
    cl: usize,
    cr: usize,
}

impl Mo {
    // [0] change vals type (if necessary) !!!!!!
    pub fn new(vals: &Vec<usize>) -> Self {
        Mo {
            vals: vals.clone(),
            cl: 0,
            cr: 0,
            // [1] add info for ans !!!!!!!
            // cnts: vec![0; vals.len()],
            // pair_cnt: 0,
        }
    }

    // [2] impl this !!!!!!!
    // update info with self.vals[i]
    fn add(&mut self, i: usize) {
        // let v = self.vals[i];
        // self.cnts[v] += 1;
        // if self.cnts[v] % 2 == 0 {
        //     self.pair_cnt += 1;
        // }
    }

    // [3] impl this !!!!!!!
    // update info without self.vals[i]
    fn delete(&mut self, i: usize) {
        // let v = self.vals[i];
        // self.cnts[v] -= 1;
        // if self.cnts[v] % 2 == 1 {
        //     self.pair_cnt -= 1;
        // }
    }

    fn process_query(&mut self, l: usize, r: usize) {
        while l < self.cl {
            self.cl -= 1;
            self.add(self.cl);
        }
        while self.cr < r {
            self.add(self.cr);
            self.cr += 1;
        }
        while self.cl < l {
            self.delete(self.cl);
            self.cl += 1;
        }
        while r < self.cr {
            self.cr -= 1;
            self.delete(self.cr);
        }
        self.cl = l;
        self.cr = r;
    }

    // [4] (if necessary) change ans vec type !!!!!!!!!!!!
    pub fn process_all(&mut self, queries: &Vec<(usize, usize)>) -> Vec<bool> {
        let bucket_size = self.vals.len().sqrt() + 1;
        let mut buckets = vec![vec![]; bucket_size + 1];
        for (qi, &(l, r)) in queries.iter().enumerate() {
            buckets[l / bucket_size].push((l, r, qi));
        }
        for (i, bucket) in buckets.iter_mut().enumerate() {
            bucket.sort_by_key(|v| v.1);
            if i % 2 == 1 {
                bucket.reverse();
            }
        }

        // [5] init ans vec !!!!!!!!!!!!
        let mut ans = vec![false; queries.len()];

        for bucket in &buckets {
            for &(l, r, qi) in bucket {
                self.process_query(l, r);
                // [6] set ans !!!!!!!!!
                ans[qi] = self.ng_cnt == 0;
            }
        }
        // [7] return ans !!!!!!
        ans
    }
}

// #[fastout]
fn main() {
    input! {
        n: usize,
        cl: [Usize1 ;n],
        q: usize,
        lrs: [(Usize1, usize); q]
    }
    let mut mo = Mo::new(&cl);
    let ans = mo.process_all(lrs);
    for &a in &ans {
        println!("{}", a);
    }
}
