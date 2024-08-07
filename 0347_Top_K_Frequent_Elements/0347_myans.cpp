class Solution
{
public:
    vector<int> topKFrequent(vector<int> &nums, int k)
    {
        unordered_map<int, int> frequency;

        for (int num : nums)
        {
            frequency[num]++;
        }

        auto cmp = [&](const pair<int, int> &a, const pair<int, int> &b)
        {
            return a.second < b.second;
        };

        priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(cmp)> pq(cmp);

        for (auto &entry : frequency)
        {
            pq.push(entry);
        }

        vector<int> result;
        for (int i = 0; i < k; i++)
        {
            result.push_back(pq.top().first);
            pq.pop();
        }

        return result;
    }
};