impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();

        let mut res = Vec::with_capacity(m*n);

        for sum in 0..m + n - 1 {
            let start = sum.saturating_sub(n - 1);
            let end = sum.min(m - 1);

            if sum % 2 == 0 {
                for i in (start..=end).rev() {
                    res.push(mat[i][sum - i]);
                }
            } else {
                for i in start..=end {
                    res.push(mat[i][sum - i]);
                }
            }
        }
        res
    }
}