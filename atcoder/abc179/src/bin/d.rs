use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        lr: [(usize, usize); k],
    }

    let mut ans = n;

    println!("{}", ans);
}
