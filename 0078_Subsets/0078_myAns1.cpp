class Solution
{
public:
    vector<vector<int>> subsets(vector<int> &nums)
    {
        vector<vector<int>> answer(1, vector<int>{});

        int nums_size = nums.size();

        for (int i = 0; i < nums_size; i++)
        {
            int answer_size = answer.size();
            for (int j = 0; j < answer_size; j++)
            {
                vector<int> new_answer = answer[j];
                new_answer.push_back(nums[i]);
                answer.push_back(new_answer);
            }
        }

        return answer;
    }
};