class Solution
{
public:
    vector<vector<int>> kSmallestPairs(vector<int> &nums1, vector<int> &nums2, int k)
    {
        auto compare = [&nums1, &nums2](vector<int> a, vector<int> b)
        {
            return nums1[a[0]] + nums2[a[1]] > nums1[b[0]] + nums2[b[1]];
        };

        priority_queue<vector<int>, vector<vector<int>>, decltype(compare)> q(compare);

        for (int i = 0; i < nums1.size() && i < k; i++)
        {
            vector<int> newpair{i, 0};
            q.push(newpair);
        }

        vector<vector<int>> answer;

        while (k > 0 && !q.empty())
        {
            vector<int> smallestPair = q.top();
            q.pop();
            answer.push_back({nums1[smallestPair[0]], nums2[smallestPair[1]]});
            k -= 1;
            if (smallestPair[1] + 1 < nums2.size())
            {
                q.push({smallestPair[0], smallestPair[1] + 1});
            }
        }

        return answer;
    }
};