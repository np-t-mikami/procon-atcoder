use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[char; w]; h],
    }

    let mut valid_i = Vec::new();
    for i in 0..h {
        let mut ok = true;
        for j in 0..w {
            if a[i][j] == '#' {
                ok = false;
                break;
            }
        }
        if ok {
            valid_i.push(i);
        }
    }

    let mut valid_j = Vec::new();
    for j in 0..w {
        let mut ok = true;
        for i in 0..h {
            if a[i][j] == '#' {
                ok = false;
                break;
            }
        }
        if ok {
            valid_j.push(j);
        }
    }

    for &i in valid_i.iter() {
        for &j in valid_j.iter() {
            print!("{}", a[i][j]);
        }
        println!("");
    }
}
