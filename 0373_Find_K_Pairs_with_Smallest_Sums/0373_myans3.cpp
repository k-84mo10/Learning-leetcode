#define ppi pair<int, pair<int, int>>
class Solution
{
public:
    vector<vector<int>> kSmallestPairs(vector<int> &nums1, vector<int> &nums2, int k)
    {
        priority_queue<ppi, vector<ppi>, greater<ppi>> pq;

        if (nums1.empty() || nums2.empty() || k <= 0)
            return {};

        pq.push({nums1[0] + nums2[0], {0, 0}});
        int nums1_size = nums1.size();
        int nums2_size = nums2.size();

        vector<vector<int>> answer;

        while (k > 0 && !pq.empty())
        {
            ppi p = pq.top();
            pq.pop();
            int i = p.second.first;
            int j = p.second.second;

            answer.push_back({nums1[i], nums2[j]});
            k--;

            if (i + 1 < nums1_size)
            {
                pq.push({nums1[i + 1] + nums2[j], {i + 1, j}});
            }
            if (i == 0 && j + 1 < nums2_size)
            {
                pq.push({nums1[i] + nums2[j + 1], {i, j + 1}});
            }
        }

        return answer;
    }
};
