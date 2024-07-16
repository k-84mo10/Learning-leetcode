class Solution
{
public:
    vector<vector<int>> permute(vector<int> &nums)
    {
        int nums_length = nums.size();
        vector<int> is_included(nums_length, 0);
        vector<vector<int>> answer;
        vector<int> current_perm;
        dfs(nums, nums_length, is_included, current_perm, answer);
        return answer;
    }

    void dfs(vector<int> &nums, int nums_length, vector<int> &is_included, vector<int> &current_perm, vector<vector<int>> &answer)
    {
        if (current_perm.size() == nums_length)
        {
            answer.push_back(current_perm);
            return;
        }

        for (int i = 0; i < nums_length; i++)
        {
            if (is_included[i] == 0)
            {
                is_included[i] = 1;
                current_perm.push_back(nums[i]);
                dfs(nums, nums_length, is_included, current_perm, answer);
                current_perm.pop_back();
                is_included[i] = 0;
            }
        }
    }
};