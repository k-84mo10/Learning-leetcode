class Solution
{
public:
    int maxOperations(vector<int> &nums, int k)
    {
        sort(nums.begin(), nums.end());

        int left = 0;
        int right = nums.size() - 1;
        int pair_num = 0;

        while (left < right)
        {
            if (nums[left] + nums[right] == k)
            {
                pair_num += 1;
                left += 1;
                right -= 1;
            }
            else if (nums[left] + nums[right] < k)
            {
                left += 1;
            }
            else
            {
                right -= 1;
            }
        }

        return pair_num;
    }
};