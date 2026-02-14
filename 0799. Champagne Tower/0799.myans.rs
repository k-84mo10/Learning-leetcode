impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let r = query_row as usize;
        let mut dp = vec![0f64; r+1];
        dp[0] = poured as f64;

        for i in 0..r {
            let mut next = vec![0f64; r+1];
            for j in 0..i+1 {
                let overflow = 0f64.max(dp[j] - 1f64);
                if overflow > 0f64 {
                    let half = overflow / 2f64;
                    next[j] += half;
                    next[j+1] += half; 
                }
            }
            dp = next;
        }

        dp[query_glass as usize].min(1f64)
    }
}