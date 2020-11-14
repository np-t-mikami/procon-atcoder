use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
    }

    let ans = n - a + b;

    println!("{}", ans);

}
