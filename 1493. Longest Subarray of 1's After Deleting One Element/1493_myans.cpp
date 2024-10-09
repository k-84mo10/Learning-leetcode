class Solution
{
public:
    int longestSubarray(vector<int> &nums)
    {
        int left = 0;
        int right = 0;
        int nums_size = nums.size();
        int zero_num = 0;

        int longest_subarray = 0;
        while (right < nums_size)
        {
            if (nums[right] == 0)
                zero_num++;
            if (zero_num == 0 || zero_num == 1)
            {
                longest_subarray = max(longest_subarray, right - left);
            }
            else
            {
                if (nums[left] == 0)
                    zero_num--;
                left++;
            }
            right++;
        }

        return longest_subarray;
    }
};