// @doc.begin [Rust/util] {Util}

fn char_util() {
    // @doc.subtitle {文字の距離}
    // @doc.src.begin
    let ch = 'F';
    let d = ch as u32 - 'A' as u32;
    // @doc.src.end

    // sort by ...
    input! {n: usize, al: [i64; n], bl: [i64; n]}
    let mut abil = vec![];
    for i in 0..n {
        abil.push((al[i], bl[i], i + 1));
    }
    // abil.sort_by_key(|x| (-x.0 - x.1, -x.0, x.2));
    abil.sort_by(|x, y| {
        if x.0 + x.1 != y.0 + y.1 {
            (y.0 + y.1).cmp(&(x.0 + x.1))
        } else if x.0 != y.0 {
            (y.0).cmp(&x.0)
        } else {
            x.2.cmp(&y.2)
        }
    });
}

// https://atcoder.jp/contests/abc241/tasks/abc241_d
// BTreeSet で 二分探索
// range() で 取得したい範囲を指定 -> その範囲のイテレータが返ってくる
// 後ろから見たければ（大きい順から見たければ） rev()
// nth(k) で k番目の値を取得 (k: 0-indexed)
fn btreeset_bisect() {
    input! {q: usize}
    let mut st = BTreeSet::new();
    for i in 0..q {
        input! {qq: usize}
        if qq == 1 {
            input! {x: i64}
            st.insert((x, i));
        } else if qq == 2 {
            input! {x: i64, k: Usize1}
            let ans = st.range(..=(x, q)).rev().nth(k).map_or(-1, |v| v.0);
            println!("{}", ans);
        } else {
            input! {x: i64, k: Usize1}
            let ans = st.range((x, 0)..).nth(k).map_or(-1, |v| v.0);
            println!("{}", ans);
        }
    }
}
