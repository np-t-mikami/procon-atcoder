use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u128,
        a: [u128; n],
    }

    let mut ans = 0;

    let mut a_acum_map = HashMap::new();
    a_acum_map.insert(0, 1);

    let mut a_acum: u128 = 0;
    for _i in 0..n {
        a_acum = (a_acum + a[_i]) % m;
        let count = a_acum_map.entry(a_acum).or_insert(0);
        *count += 1;
    }

    for _v in a_acum_map.values() {
        ans += _v * (_v - 1) / 2;
    }

    println!("{}", ans);
}
