use proconio::{fastout, input};
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    let mut ans = 1 << 60;
    let mut a_acc = vec![0; n + 1];

    let mut left = vec![1; n];
    let mut right = vec![3; n];

    for i in 1..(n + 1) {
        a_acc[i] = a_acc[i - 1] + a[i - 1];
    }

    for i in 2..(n - 1) {
        // 1st cut when 2nd cut fixed as i.
        for j in left[i - 1]..i {
            if (2 * a_acc[j] - a_acc[i]).abs() < (2 * a_acc[j + 1] - a_acc[i]).abs() {
                left[i] = j;
                break;
            }
        }
        // 3rd cut when 2nd cut fixed as i.
        right[i] = i + 1;
        for j in max(i + 1, right[i - 1])..n {
            if (a_acc[n] + a_acc[i] - 2 * a_acc[j]).abs()
                < (a_acc[n] + a_acc[i] - 2 * a_acc[j + 1]).abs()
            {
                right[i] = j;
                break;
            }
        }

        // check
        let mut sums = vec![
            a_acc[left[i]],
            a_acc[i] - a_acc[left[i]],
            a_acc[right[i]] - a_acc[i],
            a_acc[n] - a_acc[right[i]],
        ];
        sums.sort();
        ans = min(ans, sums[3] - sums[0]);
    }

    println!("{}", ans);
}
