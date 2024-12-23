class Solution {
public:
    int numTilings(int n) {
        const int Modulo = 1e9+7;

        std::vector<long> ways(n+1, 0);
        std::vector<long> prefix_sum(n+1, 0);

        if (n == 1) return 1;

        prefix_sum[0] = 1;
        ways[1] = 1;
        prefix_sum[1] = ways[1] + prefix_sum[0];
        ways[2] = ways[1] + 1;
        prefix_sum[2] = prefix_sum[1] + ways[2];

        for (int i = 3; i < n+1; i++) {
            ways[i] = (ways[i-1] + ways[i-2] + 2*prefix_sum[i-3]) % Modulo;
            prefix_sum[i] = prefix_sum[i-1] + ways[i];
        }

        return ways[n]; 
    }
};