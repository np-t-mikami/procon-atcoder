use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: [String; n],
    }

    fn _first(s: &String) -> char {
        s.chars().next().unwrap()
    }

    fn _last(s: &String) -> char {
        s.chars().last().unwrap()
    }

    let mut ok = true;

    let mut _set = HashSet::new();

    _set.insert(&l[0]);

    for i in 1..n {
        if _last(&l[i - 1]) != _first(&l[i]) || _set.contains(&l[i]) {
            ok = false;
            break;
        }
        _set.insert(&l[i]);
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
