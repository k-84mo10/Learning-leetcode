class Solution
{
public:
    void nextPermutation(vector<int> &nums)
    {
        int nums_size = nums.size();

        priority_queue<int, vector<int>, greater<int>> save_nums_by_ascending_order;

        int current_max = nums[nums_size - 1];
        save_nums_by_ascending_order.push(current_max);
        int sort_start_digit = nums_size - 1;

        for (int i = nums_size - 2; i >= 0; i--)
        {
            save_nums_by_ascending_order.push(nums[i]);
            if (nums[i] >= current_max)
            {
                current_max = nums[i];
                if (i == 0)
                {
                    for (int j = 0; j < nums_size; j++)
                    {
                        nums[j] = save_nums_by_ascending_order.top();
                        save_nums_by_ascending_order.pop();
                    }
                }
            }
            else
            {
                int j = i + 1;
                bool is_swapped = false;
                while (save_nums_by_ascending_order.size() != 0)
                {
                    int poped_num = save_nums_by_ascending_order.top();
                    if (is_swapped == false && poped_num > nums[i])
                    {
                        nums[i] = poped_num;
                        is_swapped = true;
                    }
                    else
                    {
                        if (j < nums_size)
                        {
                            nums[j] = poped_num;
                            j++;
                        }
                    }
                    save_nums_by_ascending_order.pop();
                }
                break;
            }
        }
    }
};