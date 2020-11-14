use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: isize,
        mut l: [isize; n],
    }

    let mut p = Vec::new();

    let mut ans = 0;

    for &li in l.iter() {
        if li != x {
            p.push((li - x).abs() as usize);
        }
    }

    fn factors(x: usize) -> Vec<usize> {
        let mut v = Vec::new();
        for i in 1..x {
            if i * i > x {
                break;
            } else if i * i == x {
                v.push(i);
            } else if x % i == 0 {
                v.push(i);
                v.push(x / i);
            }
        }
        v.reverse();
        v
    }

    p.sort();

    for &pi in factors(p[0]).iter() {
        let mut ok = true;
        for &qi in p.iter() {
            if qi % pi != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            ans = pi;
            break;
        }
    }

    print!("{}", ans);
}
