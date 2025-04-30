class Solution {
    public:
        int findNumbers(vector<int>& nums) {
            int count = 0;
            for (int& num : nums) {
                if (is_even(countDigits(num))) count++;
            }
            return count;
        }
    
        int countDigits(int num) {
            int digit = 0;
            while (num != 0) {
                num /= 10;
                digit += 1;
            }
            return digit;
        }
    
        bool is_even(int digit) {
            return digit % 2 == 0;
        }
    };