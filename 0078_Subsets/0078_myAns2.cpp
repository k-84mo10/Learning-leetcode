class Solution
{
    vector<vector<int>> answer;

    void chooseAddorNot(int i, vector<int> &nums, vector<int> subanswer)
    {
        int nums_size = nums.size();
        if (i >= nums_size)
        {
            answer.push_back(subanswer);
            return;
        }

        chooseAddorNot(i + 1, nums, subanswer);
        subanswer.push_back(nums[i]);
        chooseAddorNot(i + 1, nums, subanswer);
        subanswer.pop_back();
    }

public:
    vector<vector<int>> subsets(vector<int> &nums)
    {
        vector<int> subanswer;
        chooseAddorNot(0, nums, subanswer);
        return answer;
    }
};