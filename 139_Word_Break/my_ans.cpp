class Solution {
public:
    bool wordBreak(string s, vector<string>& wordDict) {
        array<bool, 1000> dp;
        fill(dp.begin(), dp.end(), false);
        dp[0] = true;

        int n = s.size();
        for (int i = 0; i <= n; i++) {
            for (string& word: wordDict) {
                int word_size = word.size();
                if (dp[i] && i+word_size <= n && s.substr(i, word_size) == word) {
                    dp[i+word_size] = true;
                }
            }
        }
        return dp[n];
    }
};