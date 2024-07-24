class Solution
{
public:
    int maxSubArray(vector<int> &nums)
    {
        // std::cin.tie(0)->sync_with_stdio(0);
        int nums_size = nums.size();
        int maximum_sum = nums[0];
        int current_sum = nums[0];

        for (int i = 1; i < nums_size; i++)
        {
            int num = nums[i];

            if (current_sum + num < num)
            {
                current_sum = num;
            }
            else
            {
                current_sum += num;
            }

            if (current_sum > maximum_sum)
            {
                maximum_sum = current_sum;
            }
        }
        return maximum_sum;
    }
};