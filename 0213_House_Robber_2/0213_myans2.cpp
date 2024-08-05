class Solution
{
public:
    int rob(vector<int> &nums)
    {
        int nums_size = nums.size();
        vector<int> dp_with_one(nums_size, 0);
        vector<int> dp_with_end(nums_size, 0);

        if (nums_size == 1)
            return nums[0];

        for (int i = 0; i < nums_size; i++)
        {
            int robbed_with_one = (i >= 2) ? dp_with_one[i - 2] + nums[i] : nums[i];
            int notrobbed_with_one = (i != 0) ? dp_with_one[i - 1] : 0;
            dp_with_one[i] = max(robbed_with_one, notrobbed_with_one);

            int robbed_with_end = (i >= 2) ? dp_with_end[i - 2] + nums[i] : nums[i];
            int notrobbed_with_end = (i != 0) ? dp_with_end[i - 1] : 0;
            dp_with_end[i] = (i != 0) ? max(robbed_with_end, notrobbed_with_end) : 0;
        }

        return max(dp_with_one[nums_size - 2], dp_with_end[nums_size - 1]);
    }
};