impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut dp: Vec<bool> = vec![false; n+1];
        dp[0] = true;

        for i in 0..n {
            if !dp[i] {
                continue;
            } 

            for word in &word_dict {
                let len = word.len();
                if i+len <= n && &s[i..i+len] == word {
                    dp[i+len] = true;
                }
            }
        }

        return dp[n];
    }
}