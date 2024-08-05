class Solution
{
public:
    int minSubArrayLen(int target, vector<int> &nums)
    {
        int left = 0;
        int right = 0;
        int nums_size = nums.size();

        int current_sum = 0;
        int minimal_length = 100001;

        while (right < nums_size)
        {
            current_sum += nums[right];
            if (current_sum >= target)
            {
                while (current_sum >= target && right >= left)
                {
                    current_sum -= nums[left];
                    left++;
                }
                minimal_length = min(2 + right - left, minimal_length);
            }
            right++;
        }

        return (minimal_length != 100001) ? minimal_length : 0;
    }
};