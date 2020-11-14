use proconio::{fastout, input};
use std::cmp::max;
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    // greatest coordinate, using a_1, ..., a_i
    let mut _max_d = Vec::new();
    // accumulation of a
    let mut _acc_a = Vec::new();
    _acc_a.push(0);

    let mut _max_d_i = 0_i64;
    let mut _d = 0_i64;
    for &_a in a.iter() {
        _d += _a;
        if _d > _max_d_i {
            _max_d_i = _d;
        }
        _max_d.push(_max_d_i);
        _acc_a.push(_d);
    }

    let mut ans = 0;

    _d = 0;
    for i in 0..n {
        let d1 = _d;
        let d2 = _d + _max_d[i];
        _d += _acc_a[i + 1];
        let d3 = _d;

        ans = max(ans, max(max(d1, d2), d3));
    }

    println!("{}", ans);
}
