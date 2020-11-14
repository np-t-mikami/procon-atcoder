use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        k: usize,
    }

    let mut i = 0;
    let j = loop {
        if i >= s.len() {
            break i;
        }
        if s[i] != '1' {
            break i;
        }
        i += 1;
    };

    let ans = if k < j + 1 { '1' } else { s[i] };
    println!("{}", ans);
}
