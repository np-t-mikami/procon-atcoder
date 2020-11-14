use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    // dp[n][X] := the number of sequence y[i] i=1...n s.t.
    //    y[1]*a[1] + ... + y[n]*a[n] = X
    //    -a[i+1]/a[i] < y[i] < a[i+1]/a[i]
    //
    // dp[n][X] = sum_[y=min]^[y=max]  dp[n-1][X-y]
    //
    let mut ans = 0;

    println!("{}", ans);
}
