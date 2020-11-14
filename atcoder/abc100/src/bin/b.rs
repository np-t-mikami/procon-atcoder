use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
    }

    let mut base = 1;
    for _ in 0..d {
        base *= 100;
    }

    let ans = if n < 100 {
        base * n
    } else {
        base * (1 + n)
    };

    println!("{}", ans);
}
