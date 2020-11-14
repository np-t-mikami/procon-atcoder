use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    fn num_divisors(n: usize) -> usize {
        let mut ret = 0;
        for i in 1..=n {
            if n % i == 0 {
                ret += 1;
            }
        }
        ret
    }

    let mut ans = 0;
    for i in 1..=n {
        if i & 1 == 0 {
            continue;
        }
        ans += if num_divisors(i) == 8 { 1 } else { 0 };
    }

    println!("{}", ans);
}
