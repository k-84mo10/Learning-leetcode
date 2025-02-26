class Solution {
public:
    int maxAbsoluteSum(vector<int>& nums) {
        int plus_sum = 0;
        int minus_sum = 0;
        int max_absolute_sum = 0;
        for (int& num : nums) {
            plus_sum = (plus_sum + num >= 0) ? plus_sum+num : 0;
            minus_sum = (minus_sum + num <= 0) ? minus_sum+num : 0;
            max_absolute_sum = max(max_absolute_sum, max(minus_sum*(-1), plus_sum));
        }
        return max_absolute_sum;
    }
};
