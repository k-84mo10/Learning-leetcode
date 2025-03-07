class Solution {
    public:
        vector<int> closestPrimes(int left, int right) {
            vector<bool> is_prime = sieve(right);
            vector<int> primes;
    
            // 指定範囲の素数を取得
            for (int i = left; i <= right; i++) {
                if (is_prime[i]) primes.push_back(i);
            }
    
            if (primes.size() < 2) return {-1, -1};
            if (left <= 2 && right >= 3) return {2, 3};
    
            // 最も近い素数ペアを探索
            int closest1 = -1, closest2 = -1;
            int min_distance = INT_MAX;
    
            for (size_t i = 1; i < primes.size(); i++) {
                int dist = primes[i] - primes[i - 1];
                if (dist == 2) return {primes[i-1], primes[i]};
                if (dist < min_distance) {
                    min_distance = dist;
                    closest1 = primes[i - 1];
                    closest2 = primes[i];
                }
            }
    
            return {closest1, closest2};
        }
    
    private:
        vector<bool> sieve(int n) {
            vector<bool> is_prime(n + 1, true);
            is_prime[0] = is_prime[1] = false;
    
            for (int p = 2; p * p <= n; p++) {
                if (!is_prime[p]) continue;
                for (int q = 2 * p; q <= n; q += p) {
                    is_prime[q] = false;
                }
            }
            return is_prime;
        }
    };
    