class Solution
{
public:
    void nextPermutation(vector<int> &nums)
    {

        int nums_size = nums.size();
        int sort_start_pos = nums_size - 1;

        for (int i = nums_size - 2; i >= 0; i--)
        {
            if (nums[i] < nums[i + 1])
            {
                sort_start_pos = i;
                break;
            }
        }

        if (sort_start_pos == nums_size - 1)
        {
            reverse(nums.begin(), nums.end());
            return;
        }

        sort(nums.begin() + sort_start_pos + 1, nums.end());

        int left = sort_start_pos + 1;
        int right = nums_size - 1;
        int next_larger = nums_size - 1;
        while (left <= right)
        {
            int medium = left + (right - left) / 2;
            if (nums[medium] > nums[sort_start_pos])
            {
                next_larger = medium;
                right = medium - 1;
            }
            else
            {
                left = medium + 1;
            }
        }

        swap(nums[sort_start_pos], nums[next_larger]);
    }
};