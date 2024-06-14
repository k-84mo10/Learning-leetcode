class Solution {
public:
    int findMin(vector<int>& nums) {
        int left = 0;
        int right = nums.size()-1;
        
        while (left < right) {
            int medium = (left + right) / 2;
            if (nums[left] < nums[right]) return nums[left];
            if (nums[right] < nums[medium]) {
                left = medium + 1;
            } else {
                right = medium;
            }

        }
        return nums[left];
    }
};