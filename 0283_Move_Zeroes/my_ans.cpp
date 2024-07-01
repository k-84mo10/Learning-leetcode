class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        int i = 0;
        int j = 1;
        int nums_size = nums.size();
        for (int i = 0; i < nums_size-1; i++) {
            if (i > j) j = i + 1;
            if (nums[i] == 0) {
                while(nums[j] == 0) {
                    if (j == nums_size-1) return ;
                    j++;
                }
                swap(nums[i], nums[j]);
            }
        }
    }
};