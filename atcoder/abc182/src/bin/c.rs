use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    fn _char_to_int(c: char) -> i64 {
        c as i64 - '0' as i64
    }
    // # of +1's mod 3
    let mut _l = 0_i64;

    // # of -1's mod 3
    let mut _m = 0_i64;
    let mut _sum_mod3 = 0;
    for _c in s.chars() {
        let _d = _char_to_int(_c);
        if _d % 3 == 1 {
            _l += 1;
        } else if _d % 3 == 2 {
            _m += 1;
        }

        _sum_mod3 = (_sum_mod3 + _d) % 3;
    }

    let mut ans;

    if _sum_mod3 == 2 {
        if _m >= 1 {
            ans = 1;
        } else if _l >= 2 {
            ans = 2;
        } else {
            ans = -1;
        }
    } else if _sum_mod3 == 1 {
        if _l >= 1 {
            ans = 1;
        } else if _m >= 2 {
            ans = 2;
        } else {
            ans = -1;
        }
    } else {
        ans = 0;
    }

    if ans == s.chars().count() as i64 {
        ans = -1;
    }

    println!("{}", ans);
}
