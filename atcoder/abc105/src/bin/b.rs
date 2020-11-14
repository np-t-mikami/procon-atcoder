use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ok = false;

    let mut x = 0;
    while x <= n {
        if (n - x) % 4 == 0 {
            ok = true;
            break;
        }
        x += 7;
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
