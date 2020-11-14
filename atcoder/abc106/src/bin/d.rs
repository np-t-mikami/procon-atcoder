use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        lr: [(usize, usize); m],
        pq: [(usize, usize); q]
    }
    const DIM: usize = 501;
    let mut lr_matrix = [[0 as usize; DIM]; DIM];

    for &(_l, _r) in lr.iter() {
        lr_matrix[_l][_r] += 1;
    }

    for _i in 0..DIM {
        for _j in 1..DIM {
            lr_matrix[_i][_j] += lr_matrix[_i][_j - 1];
        }
    }

    for &(pi, qi) in pq.iter() {
        let mut ans = 0;
        for _i in pi..=qi {
            ans += lr_matrix[_i][qi] - lr_matrix[_i][pi - 1];
        }
        println!("{}", ans);
    }
}
