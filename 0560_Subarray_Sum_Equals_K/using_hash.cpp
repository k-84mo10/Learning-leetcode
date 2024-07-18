class Solution
{
public:
    int subarraySum(vector<int> &nums, int k)
    {
        unordered_map<int, int> number_of_cumulative_sum;
        int nums_len = nums.size();
        vector<int> cumulative_sum(nums_len, 0);
        int count = 0;

        cumulative_sum[0] = nums[0];
        for (int i = 1; i < nums_len; i++)
        {
            cumulative_sum[i] = nums[i] + cumulative_sum[i - 1];
        }

        for (int i = 0; i < nums_len; i++)
        {
            if (cumulative_sum[i] == k)
                count++;
            if (number_of_cumulative_sum.find(cumulative_sum[i] - k) != number_of_cumulative_sum.end())
            {
                count += number_of_cumulative_sum[cumulative_sum[i] - k];
            }
            number_of_cumulative_sum[cumulative_sum[i]]++;
        }
        return count;
    }
};