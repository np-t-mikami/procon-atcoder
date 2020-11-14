use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
    }

    let mut ans = String::from("");
    let mut _n = n;

    while _n != 0 {
        ans.push_str(if _n % 2 == 0 {
            "0"
        } else {
            _n -= 1;
            "1"
        });
        _n /= -2;
    }

    if ans == "" {
        ans.push_str("0");
    }

    println!("{}", ans.chars().rev().collect::<String>());
}
