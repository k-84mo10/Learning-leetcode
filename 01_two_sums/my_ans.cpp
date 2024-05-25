class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        vector<int> answer;
        for (int i = 0; i < nums.size(); ++i) 
        {
            for (int j = i+1; j < nums.size(); ++j) 
            {
                if (nums[i] + nums[j] == target)
                {
                    answer.insert(answer.end(), i);
                    answer.insert(answer.end(), j);
                    return answer;
                }
            }
        }
        return answer;
    }
};