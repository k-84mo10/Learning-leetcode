class Solution
{
public:
    vector<int> intersection(vector<int> &nums1, vector<int> &nums2)
    {
        unordered_map<int, bool> is_unique;

        for (int &num : nums1)
        {
            is_unique[num] = true;
        }

        vector<int> answer;
        for (int &num : nums2)
        {
            if (is_unique.find(num) != is_unique.end())
            {
                if (is_unique[num] == true)
                {
                    answer.push_back(num);
                    is_unique[num] = false;
                }
            }
        }
        return answer;
    }
};