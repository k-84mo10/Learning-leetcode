class Solution
{
public:
    int maxArea(vector<int> &height)
    {
        int left = 0;
        int right = height.size() - 1;

        int maxarea = 0;

        while (left < right)
        {
            maxarea = max(maxarea, min(height[left], height[right]) * (right - left));
            if (height[left] < height[right])
            {
                left = left + 1;
            }
            else
            {
                right = right - 1;
            }
        }

        return maxarea;
    }
};