use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut a_min = 1 << 60;
    let mut a_max = 0;

    for ai in a {
        if ai < a_min {
            a_min = ai;
        }
        if ai > a_max {
            a_max = ai;
        }
    }

    let ans = a_max - a_min;

    println!("{}", ans);
}
