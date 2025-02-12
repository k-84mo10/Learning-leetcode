class Solution {
    public:
        int maximumSum(vector<int>& nums) {
            std::unordered_map<int, int> maxbySumDigits;
    
            int maxSum = -1;
            for (int& num : nums) {
                int sumDigits = sumofDigits(num);
                if (maxbySumDigits.find(sumDigits) == maxbySumDigits.end()) {
                    maxbySumDigits[sumDigits] = num; 
                } else {
                    int currentMax = maxbySumDigits[sumDigits];
                    maxSum = max(currentMax+num, maxSum);
                    maxbySumDigits[sumDigits] = max(currentMax, num);
                }
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