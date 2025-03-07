class Solution {
    public:
        vector<int> closestPrimes(int left, int right) {
            std::vector<bool> is_prime(right+1, true);
    
            int num1 = -1, num2 = -1;
            int latest = -1;
            int distance = 1e9;
            is_prime[0] = is_prime[1] = false;
    
            for (int p = 2; p <= right; p++) {
                if (!is_prime[p]) continue;
    
                for (int q = p + p; q <= right; q += p) {
                    is_prime[q] = false;
                }
    
                if (p >= left) {
                    if (p - latest < distance) {
                        num1 = latest;
                        num2 = p;
                        distance = num2 - num1;
                    }
                    latest = p;
                }
            }
    
            if (num1 == -1) num2 = -1;
            return {num1, num2};
        }
    };