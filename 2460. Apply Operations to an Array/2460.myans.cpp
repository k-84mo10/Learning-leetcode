class Solution {
public:
    vector<int> applyOperations(vector<int>& nums) {
        int nums_size = nums.size();
        std::vector<int> answer(nums_size, 0);

        int index = 0;
        for (int i = 0; i < nums_size; i++) {
            if (nums[i] == 0) continue;
            if (i+1 != nums_size && nums[i] == nums[i+1]) {
                answer[index] = nums[i]*2;
                nums[i+1] = 0;
            } else {
                answer[index] = nums[i];
            }
            index++;
        }
        return answer;
    }
};
