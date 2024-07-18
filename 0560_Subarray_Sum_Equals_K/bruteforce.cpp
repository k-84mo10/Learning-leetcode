class Solution
{
public:
    int subarraySum(vector<int> &nums, int k)
    {
        int nums_len = nums.size();
        int count = 0;

        for (int i = 0; i < nums_len; i++)
        {
            int sum = 0;
            for (int j = i; j < nums_len; j++)
            {
                sum += nums[j];
                if (sum == k)
                {
                    count++;
                }
            }
        }
        return count;
    }
};