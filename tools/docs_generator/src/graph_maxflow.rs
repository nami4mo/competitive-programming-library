#![allow(unused_imports, dead_code, unused_macros)]
use std::collections::VecDeque;

use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    from: usize,
    to: usize,
    rev: usize,
    cap: u64,
    flow: u64,
}

pub struct MaxFlowDinic {
    n: usize,
    edges: Vec<Vec<Edge>>,
    edges_pos: Vec<(usize, usize)>,
}

impl MaxFlowDinic {
    pub fn new(n: usize) -> Self {
        let edges = vec![vec![]; n];
        MaxFlowDinic {
            n,
            edges,
            edges_pos: vec![],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: u64) -> usize {
        let to_rev = self.edges[to].len();
        let from_rev = self.edges[from].len();
        self.edges_pos.push((from, self.edges[from].len()));
        self.edges[from].push(Edge {
            from,
            to,
            cap,
            rev: to_rev,
            flow: 0,
        });
        self.edges[to].push(Edge {
            from,
            to: from,
            cap: 0,
            rev: from_rev,
            flow: 0,
        });
        self.edges_pos.len() - 1
    }

    pub fn get_edge(&self, i: usize) -> Edge {
        assert!(i < self.edges_pos.len());
        let pos = &self.edges_pos[i];
        let mut edge = self.edges[pos.0][pos.1].clone();
        let rev_edge = &self.edges[edge.to][edge.rev];
        edge.cap += rev_edge.cap;
        edge.flow += rev_edge.cap;
        edge
    }

    pub fn edges(&self) -> Vec<Edge> {
        (0..self.edges_pos.len())
            .map(|x| self.get_edge(x))
            .collect()
    }

    pub fn flow(&mut self, s: usize, t: usize) -> u64 {
        let mut flow = 0;
        loop {
            let level = self.bfs(s);
            if level[t] < 0 {
                break;
            }
            let mut iter = vec![0; self.n];
            loop {
                let f = self.dfs(s, t, std::u64::MAX, &mut iter, &level);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }

    pub fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut vis = vec![false; self.n];
        let mut q = VecDeque::new();
        vis[s] = true;
        q.push_back(s);
        while let Some(poped) = q.pop_front() {
            for &edge in &self.edges[poped] {
                if !vis[edge.to] && edge.cap > 0 {
                    vis[edge.to] = true;
                    q.push_back(edge.to);
                }
            }
        }
        vis
    }

    fn bfs(&self, s: usize) -> Vec<i64> {
        let mut level = vec![-1; self.n];
        let mut q = VecDeque::new();
        q.push_back(s);
        level[s] = 0;
        while let Some(poped) = q.pop_front() {
            for edge in &self.edges[poped] {
                if level[edge.to] == -1 && edge.cap > 0 {
                    level[edge.to] = level[poped] + 1;
                    q.push_back(edge.to);
                }
            }
        }
        level
    }

    fn dfs(&mut self, v: usize, t: usize, up: u64, ite: &mut Vec<usize>, level: &Vec<i64>) -> u64 {
        if v == t {
            return up;
        };
        while ite[v] < self.edges[v].len() {
            let e = self.edges[v][ite[v]];
            let up = up.min(e.cap);
            if up > 0 && level[v] < level[e.to] {
                let d = self.dfs(e.to, t, up, ite, level);
                if d > 0 {
                    let rev = self.edges[v][ite[v]].rev;
                    self.edges[v][ite[v]].cap -= d;
                    self.edges[e.to][rev].cap += d;
                    return d;
                }
            }
            ite[v] += 1;
        }
        0
    }
}

// https://atcoder.jp/contests/abc010/tasks/abc010_4/
// #[fastout]
fn main() {
    input! {n: usize, g: usize, e: usize}
    let mut graph = MaxFlowDinic::new(n + 1);
    for _ in 0..g {
        input! {p: usize}
        graph.add_edge(p, n, 1);
    }
    for _ in 0..e {
        input! {a: usize, b: usize}
        graph.add_edge(a, b, 1);
        graph.add_edge(b, a, 1);
    }
    let ans = graph.flow(0, n);
    println!("{}", ans);
}
