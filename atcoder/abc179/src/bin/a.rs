use proconio::{fastout, input};


#[fastout]
fn main() {
    input! {
        s: String,
    }

    fn _last(s: &String) -> char {
        s.chars().last().unwrap()
    }

    let suffix = if _last(&s) == 's' { "es" } else { "s" };

    let ans = format!("{}{}", s, suffix);

    println!("{}", ans);
}
