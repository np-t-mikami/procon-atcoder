use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    // # of (A, B) that satisfies
    // A*B <= N-1
    let mut ans = 0;
    for _a in 1..n {
        ans += (n - 1) / _a;
    }

    println!("{}", ans);
}
