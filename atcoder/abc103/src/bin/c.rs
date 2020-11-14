use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;

    for ai in a.iter() {
        ans += ai - 1;
    }

    println!("{}", ans);
}
