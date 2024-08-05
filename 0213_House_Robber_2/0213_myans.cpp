class Solution
{
public:
    int rob(vector<int> &nums)
    {
        int nums_size = nums.size();
        vector<int> dp1(nums_size, 0);
        vector<int> dp2(nums_size, 0);

        if (nums_size == 1)
            return nums[0];

        for (int i = 0; i < nums_size - 1; i++)
        {
            int robbed = (i >= 2) ? dp1[i - 2] + nums[i] : nums[i];
            int notrobbed = (i != 0) ? dp1[i - 1] : 0;
            dp1[i] = max(robbed, notrobbed);
        }

        for (int i = 1; i < nums_size; i++)
        {
            int robbed = (i >= 2) ? dp2[i - 2] + nums[i] : nums[i];
            int notrobbed = (i != 0) ? dp2[i - 1] : 0;
            dp2[i] = max(robbed, notrobbed);
        }
        return max(dp1[nums_size - 2], dp2[nums_size - 1]);
    }
};