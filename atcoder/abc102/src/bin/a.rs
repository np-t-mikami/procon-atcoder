use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let ans = if n & 1 == 1 { 2 * n } else { n };

    println!("{}", ans);
}
