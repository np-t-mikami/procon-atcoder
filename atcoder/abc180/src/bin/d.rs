use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u128,
        y: u128,
        a: u128,
        b: u128,
    }

    let mut _str = x;
    let mut _exp = 0;
    let mut _evolve = false;

    // go to *A gym p times.
    // A^p * X < A^(p-1) * X + B
    // A^(p-1) * X < B/(A-1)
    let rhs = b / (a - 1);
    while _str < rhs {
        _str *= a;
        if _str >= y {
            _evolve = true;
            break;
        }
        _exp += 1;
    }

    // go to +B gym q times
    // A^p * X + q*B < Y
    if !_evolve {
        _exp += (y - 1 - _str) / b;
    }

    println!("{}", _exp);
}
