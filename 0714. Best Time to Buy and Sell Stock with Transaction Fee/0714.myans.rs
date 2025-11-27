impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let days = prices.len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; days+1]; days+1];

        for buy in 0..days {
            for sell in buy..days {
                dp[buy+1][sell+1] = dp[buy][sell+1]
                    .max(dp[buy+1][sell])
                    .max(dp[buy][buy] + prices[sell] - prices[buy] - fee);
            }
        }

        dp[days][days]
    }
}