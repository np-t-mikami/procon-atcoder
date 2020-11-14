use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x1: (isize, isize),
        x2: (isize, isize),
    }

    let d = (x2.0 - x1.0, x2.1 - x1.1);
    let x3 = (x2.0 - d.1, x2.1 + d.0);
    let x4 = (x1.0 - d.1, x1.1 + d.0);
    println!("{} {} {} {}", x3.0, x3.1, x4.0, x4.1);
}
