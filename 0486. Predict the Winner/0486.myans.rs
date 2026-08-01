impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            dp[i][i] = nums[i];
        }

        for distance in 1..n {
            for l in 0..n-distance {
                let r = l + distance;
                dp[l][r] = (nums[r] - dp[l][r-1]).max(nums[l] - dp[l+1][r]);
            }
        }
        
        dp[0][n-1] >= 0
    }
}