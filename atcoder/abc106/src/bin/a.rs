use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans = (a - 1) * (b - 1);

    println!("{}", ans);
}
