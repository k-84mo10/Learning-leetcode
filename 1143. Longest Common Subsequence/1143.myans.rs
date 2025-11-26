impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let t1:Vec<char> = text1.chars().collect();
        let t2:Vec<char> = text2.chars().collect();

        let t1_size = t1.len();
        let t2_size = t2.len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; t2_size+1]; t1_size+1];

        for i in 0..t1_size {
            for j in 0..t2_size {
                dp[i+1][j+1] = if t1[i] == t2[j] {
                    dp[i][j] + 1
                } else {
                    dp[i+1][j].max(dp[i][j+1])
                };
            }
        } 

        dp[t1_size][t2_size]
    }
}