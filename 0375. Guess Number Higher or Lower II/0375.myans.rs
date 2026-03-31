impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let nu = n as usize;
        let mut dp = vec![vec![0; nu+1]; nu+1];

        for len in 2..=nu {
            for l in 1..=nu-len+1 {
                let r = l + len - 1;

                let mid = (l + r) / 2;
                let mut best = i32::MAX;
                for x in mid..r {
                    let cost = x as i32 + dp[l][x-1].max(dp[x+1][r]);
                    best = best.min(cost);
                }

                dp[l][r] = best;
            }
        }

        dp[1][nu]
    }
}