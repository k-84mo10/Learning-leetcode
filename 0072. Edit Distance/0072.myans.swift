class Solution {
    func minDistance(_ word1: String, _ word2: String) -> Int {
        let m = word1.count
        let n = word2.count

        if m == 0 { return n }
        if n == 0 { return m }

        let chars1 = Array(word1)
        let chars2 = Array(word2)

        var dp = Array(repeating: Array(repeating: 0, count: n+1), count: m+1)
        
        for i in 0...m {
            dp[i][0] = i
        }
        for j in 0...n {
            dp[0][j] = j
        }

        for i in 1...m {
            for j in 1...n {
                if chars1[i-1] == chars2[j-1] {
                    dp[i][j] = dp[i-1][j-1]
                } else {
                    dp[i][j] = min(
                        dp[i-1][j] + 1,
                        dp[i][j-1] + 1,
                        dp[i-1][j-1] + 1
                    )
                }
            }
        }

        return dp[m][n]
    }
}