use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: u128,
    }

    fn sn(n: u128) -> u128 {
        let mut _n = n;
        let mut ret = 0;
        while _n > 0 {
            ret += _n % 10;
            _n /= 10;
        }
        ret
    }

    let mut ans: u128 = 0;
    let mut base: u128 = 1;

    for _ in 0..k {
        ans += base;
        println!("{}", ans);

        if (ans + base) / sn(ans + base) > (ans + base * 10) / sn(ans + base * 10) {
            base *= 10;
        }
    }
}
