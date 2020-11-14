use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    let n = s.len();

    let mut ok = false;

    for i in 0..n {
        ok = true;
        for j in 0..n {
            if s[j] != t[(j + i) % n] {
                ok = false;
                break;
            }
        }

        if ok {
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
