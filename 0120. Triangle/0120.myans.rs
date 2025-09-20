impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let height = triangle.len();
        let mut dp = triangle[height-1].clone();

        if height == 1 {
            return dp[0];
        }

        for r in (0..height-1).rev() {
            for c in 0..triangle[r].len() {
                dp[c] = triangle[r][c] + dp[c].min(dp[c+1]);
            }
        }

        dp[0]
    }
}