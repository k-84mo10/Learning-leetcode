class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int left = 0, right = 0;

        while (right < nums.size()) {
            while (right < nums.size() - 1 && nums[right] == nums[right+1]) right += 1;
            nums[left] = nums[right];
            left += 1;
            right += 1;
        }

        return left;
    }
};