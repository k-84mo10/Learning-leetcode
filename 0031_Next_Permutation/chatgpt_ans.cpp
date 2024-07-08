class Solution
{
public:
    void nextPermutation(vector<int> &nums)
    {
        int nums_size = nums.size();
        int i = nums_size - 2;

        // Step 1: Find the first decreasing element from the end
        while (i >= 0 && nums[i] >= nums[i + 1])
        {
            i--;
        }

        if (i >= 0)
        {
            // Step 2: Find the element just larger than nums[i]
            int j = nums_size - 1;
            while (nums[j] <= nums[i])
            {
                j--;
            }
            // Step 3: Swap them
            swap(nums[i], nums[j]);
        }

        // Step 4: Reverse the elements after index i
        reverse(nums.begin() + i + 1, nums.end());
    }
};
