impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let days = prices.len();

        let mut dp: Vec<i32> = vec![0; days+1];

        for buy in 0..days {
            let mut next: Vec<i32> = vec![0; days+1];
            for sell in buy..days {
                next[sell+1] = next[sell]
                    .max(dp[sell+1])
                    .max(dp[buy] + prices[sell] - prices[buy] - fee);
            }
            dp = next;
        }

        dp[days]
    }
}