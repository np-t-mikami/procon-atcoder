use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let n_s = s.len();
    let n_t = t.len();
    let mut dp = vec![vec![0; n_t + 1]; n_s + 1];

    for i in 0..n_s {
        for j in 0..n_t {
            if s[i] == t[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j]);
            }
        }
    }

    println!("{}", 0);
}
