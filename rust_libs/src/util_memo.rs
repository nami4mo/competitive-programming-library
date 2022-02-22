fn main() {
    // 文字の距離
    let ch = 'F';
    let d = ch as u32 - 'A' as u32;

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
