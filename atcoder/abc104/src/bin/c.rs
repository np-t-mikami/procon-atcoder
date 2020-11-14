use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        d: usize,
        g: isize,
        pc: [(isize, isize); d],
    }

    let mut ans = 1 << 40;

    for config in 0..(1 << d) {
        let mut _config = config;
        let mut _score = 0;
        let mut _num_solved = 0;
        let mut _highest_empty = 0;
        for i in 0..d {
            if _config & 1 == 1 {
                _score += pc[i].0 * 100 * (i as isize + 1) + pc[i].1;
                _num_solved += pc[i].0;
            } else {
                _highest_empty = i;
            }
            _config >>= 1;
        }

        let highest_empty_score = 100 * (_highest_empty as isize + 1);
        let _add_solved = max(
            0,
            (g - _score + highest_empty_score - 1) / highest_empty_score,
        );
        if _add_solved <= pc[_highest_empty].0 {
            ans = min(ans, _num_solved + _add_solved);
        }
    }

    println!("{}", ans);
}
