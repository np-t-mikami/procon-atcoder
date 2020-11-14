use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut x: [i64; n],
    }

    let mut d_m = 0_i64;
    let mut d_e = 0.0_f64;
    let mut d_c = 0_i64;

    for i in 0..n {
        d_m += x[i].abs();
        d_e += (x[i] * x[i]) as f64;
        d_c = if d_c < x[i].abs() { x[i].abs() } else { d_c };
    }

    d_e = d_e.sqrt();

    println!("{}\n{}\n{}", d_m, d_e, d_c);
}
