use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut sn = 0;
    let mut _n = n;

    while _n > 0 {
        sn += _n % 10;
        _n /= 10;
    }

    println!("{}", if n % sn == 0 { "Yes" } else { "No" });
}
