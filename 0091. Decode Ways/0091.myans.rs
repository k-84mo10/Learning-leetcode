impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![0; chars.len()+2];

        dp[chars.len()] = 1;

        for i in (0..chars.len()).rev() {
            match chars[i] {
                '0' => dp[i] = 0,
                '1' => dp[i] = dp[i+1] + dp[i+2],
                '2' => {
                    if i < chars.len()-1 && (chars[i+1] == '7' || chars[i+1] == '8' || chars[i+1] == '9') {
                        dp[i] = dp[i+1];
                    } else {
                        dp[i] = dp[i+1] + dp[i+2];
                    }
                },
                _ => dp[i] = dp[i+1],
            }
        }

        dp[0]
    }
}