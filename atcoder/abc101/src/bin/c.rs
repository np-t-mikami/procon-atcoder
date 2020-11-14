use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64; n],
    }

    // x =  ceil of (n-1)/(k-1)
    println!("{}", (n - 1 + k - 2) / (k - 1));
}
