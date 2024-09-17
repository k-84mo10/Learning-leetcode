class Solution
{
public:
    vector<int> productExceptSelf(vector<int> &nums)
    {
        int all_product = 1;
        int zero_num = 0;
        int last_zero_position = -1;

        for (int i = 0; i < nums.size(); i++)
        {
            if (nums[i] == 0)
            {
                last_zero_position = i;
                zero_num++;
                continue;
            }
            all_product *= nums[i];
        }

        vector<int> answer(nums.size(), 0);
        if (zero_num >= 2)
            return answer;
        if (zero_num == 1)
        {
            answer[last_zero_position] = all_product;
            return answer;
        }

        for (int i = 0; i < nums.size(); i++)
        {
            answer[i] = all_product / nums[i];
        }

        return answer;
    }
};