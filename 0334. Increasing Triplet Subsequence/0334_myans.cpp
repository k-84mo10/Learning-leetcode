class Solution
{
public:
    bool increasingTriplet(vector<int> &nums)
    {
        int smallest = numeric_limits<int>::max();
        int second_smallest = numeric_limits<int>::max();
        for (int &num : nums)
        {
            if (num <= smallest)
            {
                smallest = num;
            }
            else if (num <= second_smallest)
            {
                second_smallest = num;
            }
            else
            {
                return true;
            }
        }

        return false;
    }
};