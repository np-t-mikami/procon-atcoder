use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    let mut _max_gcd_ness = 0;

    for _i in 2..1001 {
        let mut _gcd_ness = 0;
        for &_a in a.iter() {
            if _a % _i == 0 {
                _gcd_ness += 1;
            }
        }

        if _max_gcd_ness < _gcd_ness {
            ans = _i;
            _max_gcd_ness = _gcd_ness;
        }
    }

    println!("{}", ans);
}
