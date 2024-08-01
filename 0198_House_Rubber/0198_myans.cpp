class Solution
{
public:
    int rob(vector<int> &nums)
    {
        int nums_size = nums.size();
        vector<int> dp(nums_size, 0);

        dp[0] = nums[0];
        if (nums_size == 1)
            return dp[0];

        dp[1] = max(dp[0], nums[1]);
        if (nums_size == 2)
            return dp[1];

        dp[2] = max(dp[0] + nums[2], dp[1]);

        for (int i = 3; i < nums_size; i++)
        {
            dp[i] = max(dp[i - 1], max(dp[i - 2], dp[i - 3]) + nums[i]);
        }

        return dp[nums_size - 1];
    }
};