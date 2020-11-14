use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [i64; 3*n],
    }

    let mut ans = 0;

    for c in 0..8 {
        let mut vec = Vec::new();
        let mut sign = Vec::new();
        let mut _c = c;
        for _ in 0..3 {
            sign.push(
                if _c & 1 == 0 {
                    1
                } else {
                    -1
                }
            );
            _c = _c >> 1;
        }

        for i in 0..n {
            let mut v = 0;
            for j in 0..3 {
                v += x[3*i + j] * sign[j];
            }
            vec.push(v);
        }
        vec.sort();

        let mut _ans = 0;
        for i in 0..m {
            _ans += vec[n-1-i];
        }

        if _ans > ans {
            ans = _ans;
        }
    }

    println!("{}", ans);
}
