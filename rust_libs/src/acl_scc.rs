//https://github.com/rust-lang-ja/ac-library-rs

// @doc.begin [Rust/scc] {SCC (ACL-RS)}
// @doc.text.inline Thanks! [ac-library-rs](https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/scc.rs)
// @doc.src_c.begin
//https://github.com/rust-lang-ja/ac-library-rs

pub mod internal_scc {
    pub struct Csr<E> {
        start: Vec<usize>,
        elist: Vec<E>,
    }

    impl<E> Csr<E>
    where
        E: Copy,
    {
        pub fn new(n: usize, edges: &[(usize, E)], init: E) -> Self {
            let mut csr = Csr {
                start: vec![0; n + 1],
                elist: vec![init; edges.len()],
            };
            for e in edges.iter() {
                csr.start[e.0 + 1] += 1;
            }
            for i in 1..=n {
                csr.start[i] += csr.start[i - 1];
            }
            let mut counter = csr.start.clone();
            for e in edges.iter() {
                csr.elist[counter[e.0]] = e.1;
                counter[e.0] += 1;
            }
            csr
        }
    }

    #[derive(Copy, Clone)]
    struct _Edge {
        to: usize,
    }

    /// Reference:
    /// R. Tarjan,
    /// Depth-First Search and Linear Graph Algorithms
    pub struct SccGraph {
        n: usize,
        edges: Vec<(usize, _Edge)>,
    }

    impl SccGraph {
        pub fn new(n: usize) -> Self {
            SccGraph { n, edges: vec![] }
        }

        pub fn num_vertices(&self) -> usize {
            self.n
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.edges.push((from, _Edge { to }));
        }

        /// return pair of (# of scc, scc id)
        pub fn scc_ids(&self) -> (usize, Vec<usize>) {
            // In C++ ac-library, this function is implemented by using recursive lambda functions.
            // Instead, we use fn and struct for capturing environments.
            struct _Env {
                g: Csr<_Edge>,
                now_ord: usize,
                group_num: usize,
                visited: Vec<usize>,
                low: Vec<usize>,
                ord: Vec<Option<usize>>,
                ids: Vec<usize>,
            }
            let mut env = _Env {
                g: Csr::new(self.n, &self.edges, _Edge { to: 0 }),
                now_ord: 0,
                group_num: 0,
                visited: Vec::with_capacity(self.n),
                low: vec![0; self.n],
                ord: vec![None; self.n],
                ids: vec![0; self.n],
            };

            fn dfs(v: usize, n: usize, env: &mut _Env) {
                env.low[v] = env.now_ord;
                env.ord[v] = Some(env.now_ord);
                env.now_ord += 1;
                env.visited.push(v);

                for i in env.g.start[v]..env.g.start[v + 1] {
                    let to = env.g.elist[i].to;
                    if let Some(x) = env.ord[to] {
                        env.low[v] = std::cmp::min(env.low[v], x);
                    } else {
                        dfs(to, n, env);
                        env.low[v] = std::cmp::min(env.low[v], env.low[to]);
                    }
                }
                if env.low[v] == env.ord[v].unwrap() {
                    loop {
                        let u = *env.visited.last().unwrap();
                        env.visited.pop();
                        env.ord[u] = Some(n);
                        env.ids[u] = env.group_num;
                        if u == v {
                            break;
                        }
                    }
                    env.group_num += 1;
                }
            }
            for i in 0..self.n {
                if env.ord[i].is_none() {
                    dfs(i, self.n, &mut env);
                }
            }
            for x in env.ids.iter_mut() {
                *x = env.group_num - 1 - *x;
            }
            (env.group_num, env.ids)
        }

        pub fn scc(&self) -> Vec<Vec<usize>> {
            let ids = self.scc_ids();
            let group_num = ids.0;
            let mut counts = vec![0usize; group_num];
            for &x in ids.1.iter() {
                counts[x] += 1;
            }
            let mut groups: Vec<Vec<usize>> = (0..ids.0).map(|_| vec![]).collect();
            for i in 0..group_num {
                groups[i].reserve(counts[i]);
            }
            for i in 0..self.n {
                groups[ids.1[i]].push(i);
            }
            groups
        }
    }
}
pub mod scc {
    use crate::internal_scc;

    pub struct SccGraph {
        internal: internal_scc::SccGraph,
    }

    impl SccGraph {
        pub fn new(n: usize) -> Self {
            SccGraph {
                internal: internal_scc::SccGraph::new(n),
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            let n = self.internal.num_vertices();
            assert!(from < n);
            assert!(to < n);
            self.internal.add_edge(from, to);
        }

        pub fn scc(&self) -> Vec<Vec<usize>> {
            self.internal.scc()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_scc_simple() {
            let mut graph = SccGraph::new(2);
            graph.add_edge(0, 1);
            graph.add_edge(1, 0);
            let scc = graph.scc();
            assert_eq!(scc.len(), 1);
        }

        #[test]
        fn test_scc_self_loop() {
            let mut graph = SccGraph::new(2);
            graph.add_edge(0, 0);
            graph.add_edge(0, 0);
            graph.add_edge(1, 1);
            let scc = graph.scc();
            assert_eq!(scc.len(), 2);
        }

        #[test]
        fn solve_alpc_g_sample1() {
            // https://atcoder.jp/contests/practice2/tasks/practice2_g
            let n: usize = 6;
            let edges = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];

            let mut graph = SccGraph::new(n);
            for (u, v) in edges.into_iter() {
                graph.add_edge(u, v);
            }

            let scc = graph.scc();
            assert_eq!(scc, vec![vec![5], vec![1, 4], vec![2], vec![0, 3]]);
        }
    }
}
use scc::*;
// @doc.src_c.end

/* @doc.text.begin
## 使い方

### 初期化

```rs
let mut sg = SccGraph::new(n); // n: ノード数
```

### 辺追加
```rs
sg.add_edge(u, v);  // u -> v の有向辺
```

### SCC
```rs
let scc: Vec<Vec<usize>> = sg.scc();
```

- 全ての頂点がちょうど1つずつ、どれかのリストに含まれます。
- 内側のリストと強連結成分が一対一に対応します。リスト内での頂点の順序は未定義です。
- リストはトポロジカルソートされています。異なる強連結成分の頂点 `u`,`v` について、`u` から `v` に到達できる時、`u` の属するリストは `v` の属するリストよりも前です。

（[ACL公式](https://atcoder.github.io/ac-library/production/document_ja/scc.html) より引用）


@doc.text.end */

// @doc.subtitle {例題}
// @doc.text.inline [ABC245-F](https://atcoder.jp/contests/abc245/tasks/abc245_f): 要素数2以上の閉路にたどり着けるノードの数を数える
// @doc.src.begin
use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashMap, HashSet};

// https://atcoder.jp/contests/abc245/tasks/abc245_f
// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
    }

    // グラフを逆順にして SCC
    // 要素数 2 以上のクラスタから到達可能なノードが、条件を満たす

    // 通常グラフ と SCC 用グラフの構築（この問題では辺の向きを逆にする）
    let mut gl = vec![vec![]; n]; // reversed graph
    let mut sg = SccGraph::new(n); // reversed graph
    for &(u, v) in &es {
        gl[v].push(u);
        sg.add_edge(v, u);
    }

    // SCC 実行
    let scc = sg.scc();

    // 各ノードが属するクラスターの HashMap 作成
    let node_to_cluster = scc
        .iter()
        .enumerate()
        .flat_map(|(i, nodes)| nodes.iter().map(|&v| (v, i)).collect::<Vec<(_, _)>>())
        .collect::<HashMap<usize, usize>>();

    // 各クラスターをノードとみなしたグラフの作成
    let scc_gl = {
        let mut scc_gl = vec![HashSet::<usize>::new(); scc.len()];
        for i in 0..n {
            let cluster = node_to_cluster[&i];
            for &node in &gl[i] {
                scc_gl[cluster].insert(node_to_cluster[&node]);
            }
        }
        scc_gl
            .into_iter()
            .map(|neibs| neibs.into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };

    // 要素数 2 以上のクラスターから辿れるクラスターを列挙する DP
    // SCC はトポロジカルソートされているので、前から配る DP が可能
    let mut dp = vec![false; scc.len()];
    for i in 0..scc.len() {
        if scc[i].len() >= 2 {
            dp[i] = true;
        }
        for &neib in &scc_gl[i] {
            dp[neib] |= dp[i];
        }
    }

    let ans = (0..scc.len())
        .map(|i| match dp[i] {
            true => scc[i].len(),
            false => 0,
        })
        .sum::<usize>();
    println!("{}", ans);
}

// @doc.src.end
// @doc.end
