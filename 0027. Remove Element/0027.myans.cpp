class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int left = 0, right = nums.size()-1;
        while (right >= 0 && nums[right] == val) right -= 1;

        while (left <= right) {
            if (nums[left] == val) {
                nums[left] = nums[right];
                nums[right] = val;
                while (nums[right] == val) right -= 1;
            }
            left += 1;
        }

        return left;
    }
};