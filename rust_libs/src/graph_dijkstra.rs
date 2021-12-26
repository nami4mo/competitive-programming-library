#![allow(unused_imports)]
use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn dijkstra(start: usize, n: usize, gl: &Vec<Vec<(usize, u64)>>) -> Vec<u64> {
    const INF: u64 = 1e18 as u64;
    let mut d = vec![INF; n];
    d[start] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));
    while let Some(Reverse(v)) = heap.pop() {
        let (dist, i) = v;
        if d[i] < dist {
            continue;
        }
        for &(to, cost) in &gl[i] {
            if d[to] > d[i] + cost {
                d[to] = d[i] + cost;
                heap.push(Reverse((d[to], to)));
            }
        }
    }
    d
}

// https://atcoder.jp/contests/typical90/tasks/typical90_m
fn main() {
    input! {n: usize, m:usize}
    let mut gl = vec![vec![]; n];
    for _ in 0..m {
        input! {a: Usize1, b: Usize1, c:u64};
        gl[a].push((b, c));
        gl[b].push((a, c));
    }
    let d1 = dijkstra(0, n, &gl);
    let d2 = dijkstra(n - 1, n, &gl);
    for i in 0..n {
        println!("{}", d1[i] + d2[i]);
    }
}
