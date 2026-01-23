class Solution {
public:
    int arrangeCoins(int n) {
        long long x = 1LL + 8LL * n;
        return ((int)std::sqrt((double)x) - 1) / 2;
    }
};