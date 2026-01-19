impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut prefix_sum: Vec<Vec<i32>> = vec![vec![0; n+1]; m+1];

        for i in 0..m {
            for j in 0..n {
                prefix_sum[i+1][j+1] = prefix_sum[i+1][j] + prefix_sum[i][j+1] - prefix_sum[i][j] + mat[i][j];
            }
        }

        let exists = |k: usize, ps: &Vec<Vec<i32>>| -> bool {
            if k == 0 { return true; }
            for i in 0..=m-k {
                let i2 = i + k;
                for j in 0..=n-k {
                    let j2 = j + k;
                    let sum = ps[i2][j2] - ps[i2][j] - ps[i][j2] + ps[i][j];
                    if sum <= threshold {
                        return true;
                    }
                }
            }
            false
        };

        let (mut lo, mut hi) = (0usize, m.min(n));
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if exists(mid, &prefix_sum) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo as i32
    }
}