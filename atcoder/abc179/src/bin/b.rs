use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n],
    }

    let mut ans = "No";

    fn _doublet(_d: (usize, usize)) -> bool {
        _d.0 == _d.1
    }

    for i in 2..n {
        if _doublet(d[i]) && _doublet(d[i-1]) && _doublet(d[i-2]) {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}
