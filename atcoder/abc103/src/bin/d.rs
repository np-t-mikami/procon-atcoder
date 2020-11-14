use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m : usize,
        mut ab: [(usize,usize); m],
    }

    let mut ans = 0;
    let mut a = [false; 100001];
    let mut b = [false; 100001];

    for _ab in ab.iter() {
        a[_ab.0] = true;
        b[_ab.1] = true;
    }

    let mut group = false;
    for _i in 0..(n + 1) {
        if group && b[_i] {
            group = false;
            ans += 1;
            // println!("debug: {}", _i);
        }
        if !group && a[_i] {
            group = true;
        }
    }

    println!("{}", ans);
}
