class Solution
{
public:
    int maxOperations(vector<int> &nums, int k)
    {
        unordered_map<int, int> num_count;
        int answer = 0;
        for (int &num : nums)
        {
            if (num_count.find(k - num) != num_count.end() && num_count[k - num] >= 1)
            {
                answer++;
                num_count[k - num]--;
            }
            else
            {
                num_count[num]++;
            }
        }
        return answer;
    }
};