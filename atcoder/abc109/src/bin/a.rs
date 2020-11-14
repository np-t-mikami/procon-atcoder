use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans = if (a * b) & 1 == 1 { "Yes" } else { "No" };

    println!("{}", ans);
}
