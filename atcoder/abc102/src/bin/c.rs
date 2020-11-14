use proconio::{fastout, input};

const fn abs(v: i64) -> i64 {
    let index = (v >= 0) as usize;
    [-v, v][index]
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    for i in 0..n {
        a[i] = a[i] - ((i + 1) as i64);
    }

    a.sort();

    let b = a[n / 2];

    let mut ans = 0;

    for ai in a {
        ans += abs(b - ai);
    }

    println!("{}", ans);
}
