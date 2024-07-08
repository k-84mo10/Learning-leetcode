class Solution
{
public:
    void nextPermutation(vector<int> &nums)
    {
        int nums_size = nums.size();
        int sort_start_position = nums_size - 1;

        for (int i = nums_size - 2; i >= 0; i--)
        {
            if (nums[i] < nums[i + 1])
            {
                sort_start_position = i;
                break;
            }
        }

        if (sort_start_position == nums_size - 1)
        {
            reverse(nums.begin(), nums.end());
            return;
        }

        sort(nums.begin() + sort_start_position + 1, nums.end());

        for (int i = sort_start_position + 1; i < nums_size; i++)
        {
            if (nums[sort_start_position] < nums[i])
            {
                swap(nums[sort_start_position], nums[i]);
                break;
            }
        }
    }
};