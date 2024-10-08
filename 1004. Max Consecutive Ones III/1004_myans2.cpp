class Solution
{
public:
    int longestOnes(vector<int> &nums, int k)
    {
        int left = 0;
        int right = 0;
        int added_one = 0;
        int max_consecutive_one = 0;

        while (right < nums.size())
        {
            if (nums[right] == 0)
                added_one++;
            if (added_one > k)
            {
                if (nums[left] == 0)
                    added_one--;
                left++;
            }
            else
            {
                max_consecutive_one = max(max_consecutive_one, right - left + 1);
            }
            right++;
        }
        return max_consecutive_one;
    }
};