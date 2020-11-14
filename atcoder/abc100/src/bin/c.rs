use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    let mut ans = 0;

    for &ai in a.iter() {
        let mut _t = ai;
        while _t & 1 == 0 {
            _t = _t >> 1;
            ans += 1;
        }
    }

    println!("{}", ans);
}
