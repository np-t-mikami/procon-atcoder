use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut factor_lower = Vec::new();
    let mut factor_higher = Vec::new();

    for i in 1..=n {
        if i * i > n {
            break;
        }

        if n % i == 0 {
            factor_lower.push(i);

            if i * i != n {
                factor_higher.push(n / i);
            }
        }
    }

    factor_higher.reverse();

    for f in factor_lower.iter() {
        print!("{}\n", f);
    }
    for f in factor_higher.iter() {
        print!("{}\n", f);
    }
}
