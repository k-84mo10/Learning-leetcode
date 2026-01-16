class Solution {
public:
    bool isPowerOfTwo(int n) {
        if (n < 0) return false;
        return 1 == std::popcount(static_cast<std::uint32_t>(n));
    }
};