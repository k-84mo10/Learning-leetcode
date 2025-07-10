impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // DP
        let m = grid.len();
        let n = grid[0].len();
        const max_value: i32 = i32::MAX;
        let mut dp = vec![vec![max_value; n]; m];

        dp[0][0] = grid[0][0];
        for i in 1..n {
            dp[0][i] = dp[0][i-1] + grid[0][i];
        } 

        for j in 1..m {
            dp[j][0] = dp[j-1][0] + grid[j][0];
            for i in 1..n {
                dp[j][i] = dp[j][i-1].min(dp[j-1][i]) + grid[j][i];
            }
        }

        dp[m-1][n-1]
    }
}