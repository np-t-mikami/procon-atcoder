use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut ok = true;

    if s[0] != 'A' {
        ok = false;
    }

    let mut count_c = 0;
    let mut index_c = 0;
    for i in 2..(s.len() - 1) {
        if s[i] == 'C' {
            count_c += 1;
            index_c = i;
        }
    }
    if count_c != 1 {
        ok = false;
    }

    for i in 0..(s.len()) {
        if i == 0 || i == index_c {
            continue;
        }
        if s[i].is_uppercase() {
            ok = false;
        }
    }

    println!("{}", if ok { "AC" } else { "WA" });
}
