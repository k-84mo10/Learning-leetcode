class Solution {
    public:
        int maximumSum(vector<int>& nums) {
            std::vector<int> maxbydigits(82, 0);
    
            int maxSum = -1;
            for (int& num : nums) {
                int sumdigits = sumofDigits(num);
                int currentMax = maxbydigits[sumdigits];
                if (currentMax != 0) maxSum = max(maxSum, currentMax+num);
                maxbydigits[sumdigits] = max(num, maxbydigits[sumdigits]);
            }        
            return maxSum;
        }
    
        int sumofDigits(int num) {
            int sum = 0;
            while (num != 0) {
                sum += num % 10;
                num /= 10;
            }
            return sum;
        }
    };