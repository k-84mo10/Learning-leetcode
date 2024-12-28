class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        if (nums.size() == 1) return 1;

        int currentNum = nums[0];
        int appearCount = 1;

        int i = 1; 
        int j = 1;

        while (i < nums.size()) {
            if (nums[i] == currentNum) {
                appearCount++;
                if (appearCount <= 2) {
                    nums[j] = nums[i];
                    j++;
                }
            } else {
                appearCount = 1;
                currentNum = nums[i];
                nums[j] = nums[i];
                j++;
            }
            i++;
        }

        return j;
    }
};