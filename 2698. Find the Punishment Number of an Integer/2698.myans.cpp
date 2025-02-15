class Solution {
    public:
        int punishmentNumber(int n) {
            int sum = 0;
            for (int i = 1; i <= n; i++) {
                if (is_punishmentNumber(i)) sum += i*i;
            }
            return sum;
        }
    
        bool is_punishmentNumber(int n) {
            int dual = n*n;
            return dns(n, dual, 0);
        }
    
        bool dns(int target, int left, int currentSum) {
            if (left == 0) {
                return (currentSum == target);
            }
            if (currentSum > target) return false;
            
            int digit = 1;
            while (left >= digit) {
                if (dns(target, left%digit, currentSum+left/digit)) return true;
                digit *= 10;
            }
            return false;
        }
    };