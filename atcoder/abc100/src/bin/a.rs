use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans = if a <= 8 && b <= 8 {
        "Yay!"
    } else {
        ":("
    };

    println!("{}", ans);
}
