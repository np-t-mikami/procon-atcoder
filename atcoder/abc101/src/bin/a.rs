use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;

    for c in s {
        ans += if c == '+' { 1 } else { -1 };
    }

    println!("{}", ans);
}
