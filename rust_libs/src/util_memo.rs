// @doc.begin [Rust/util] {Util}

// 2点を通る直線 ax + by + c = 0 を返す
// 直線の一意性を保つため、以下の条件を満たす
// - a, b, c は互いに素
// - a >= 0
// - a == 0 なら b > 0
// (ok) 3x - 4y + 2= 0, 2y - 5 = 0, 3x + 7 = 0
// (ng) -2b + 3 = 0, 2x + 4y + 6 = 0, -3x + 2y + 1 = 0
fn get_line_ax_by_c_0(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64, i64) {
    assert!(x1 != x2 || y1 != y2, "Error: (x1, y1) == (x2, y2)");
    let mut a = y2 - y1;
    let mut b = x1 - x2;
    let mut c = (y1 - y2) * x1 + (x2 - x1) * y1;
    let non0s = vec![a, b, c]
        .into_iter()
        .filter(|&v| v != 0)
        .collect::<Vec<_>>();
    let g = non0s.iter().fold(non0s[0], |acc, &v| gcd(acc, v));
    a /= g;
    b /= g;
    c /= g;
    if a < 0 || (a == 0 && b < 0) {
        a *= -1;
        b *= -1;
        c *= -1;
    }
    (a, b, c)
}

fn char_util() {
    // @doc.subtitle {文字の距離}
    // @doc.src.begin
    let ch = 'F';
    let d = ch as u32 - 'A' as u32;
    // @doc.src.end

    // @doc.subtitle {文字コード -> 文字}
    // @doc.src.begin
    let c = (2 as u8 + 'a' as u8) as char;
    let z = (25 as u8 + 'a' as u8) as char;
    assert_eq!(c, 'c');
    assert_eq!(z, 'z');
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
