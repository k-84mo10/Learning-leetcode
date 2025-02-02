class Solution {
public:
    bool check(vector<int>& nums) {
        int decrease_count = 0;
        for (int i = 1; i < nums.size(); i++) {
            if (nums[i] < nums[i-1]) {
                decrease_count ++;
            }
            if (decrease_count >= 2 || (decrease_count != 0 && nums[0] < nums[i])) return false;
        }
        return true;
    }
};
