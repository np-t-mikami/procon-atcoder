use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    let ans = (k / 2) * (k - k / 2);

    println!("{}", ans);
}
