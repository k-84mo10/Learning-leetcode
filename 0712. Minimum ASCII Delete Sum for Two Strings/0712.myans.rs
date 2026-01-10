impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let b1_len = b1.len();
        let b2_len = b2.len();

        let mut dp: Vec<i32> = vec![0; b1_len+1];

        for j in 0..b1_len {
            dp[j+1] = dp[j] + b1[j] as i32;
        }

        for i in 0..b2_len {
            let mut next: Vec<i32> = vec![0; b1_len+1];
            next[0] = dp[0] + b2[i] as i32;
            for j in 0..b1_len {
                if b2[i] == b1[j] {
                    next[j+1] = dp[j];
                } else {
                    let del2 = dp[j+1] + b2[i] as i32;
                    let del1 = next[j] + b1[j] as i32;
                    next[j+1] = del1.min(del2);
                }
            }
            dp = next;
        }

        dp[b1_len]
    }
}