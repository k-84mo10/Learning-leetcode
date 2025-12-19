impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let days = prices.len();
        let k = k as usize;
        let h = k / 2;

        let base: i64 = prices
            .iter()
            .zip(strategy.iter())
            .map(|(&p, &s)| (p*s) as i64)
            .sum();

        let first_delta: i64 = prices[..h]
            .iter()
            .zip(&strategy[..h])
            .map(|(&p, &s)| (-p*s) as i64)
            .sum();

        let last_delta: i64 = prices[h..k]
            .iter()
            .zip(&strategy[h..k])
            .map(|(&p, &s)| (p*(1-s)) as i64)
            .sum();      

        let mut delta = first_delta + last_delta;  
        let mut max_delta = delta;

        for l in 1..days-k+1 {
            // l-1 がスライド外に出る
            delta += (prices[l-1]*strategy[l-1]) as i64;

            // l+h-1 が 1→0 になる
            delta -= prices[l+h-1] as i64;

            // l+2h-1 が 1 になる
            delta += (prices[l+k-1]*(1-strategy[l+k-1])) as i64;

            max_delta = max_delta.max(delta);
        }

        base + max_delta.max(0)
    }
}