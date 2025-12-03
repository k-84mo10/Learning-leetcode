impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut dp: Vec<i32> = vec![0; cols+1];
        let mut max_len = 0;

        for row in 0..rows {
            let mut next = vec![0; cols+1];
            for col in 0..cols {
                if matrix[row][col] == '1' {
                    next[col+1] = next[col].min(dp[col+1]).min(dp[col]) + 1;
                    max_len = max_len.max(next[col+1]);
                } else {
                    next[col+1] = 0;
                }
            }
            dp = next;
        }

        max_len * max_len
    }
}