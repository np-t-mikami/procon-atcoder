use proconio::{fastout, input};

fn pow(base: usize, exp: usize) -> usize {
    let mut ret: usize = 1;
    for _ in 0..exp {
        ret *= base;
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let ans = match k & 1 {
        0 => {
            let l = k / 2;
            let m_odd = (n / l + 1) / 2;
            let m_even = (n / l) / 2;
            pow(m_odd, 3) + pow(m_even, 3)
        }
        _ => pow(n / k, 3),
    };

    println!("{}", ans);
}
