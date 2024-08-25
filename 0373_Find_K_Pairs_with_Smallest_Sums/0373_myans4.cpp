class Solution
{
public:
    vector<vector<int>> kSmallestPairs(vector<int> &nums1, vector<int> &nums2, int k)
    {
        auto compare = [&nums1, &nums2](pair<int, int> pair1, pair<int, int> pair2)
        {
            return nums1[pair1.first] + nums2[pair1.second] > nums1[pair2.first] + nums2[pair2.second];
        };

        int nums1_size = nums1.size();
        int nums2_size = nums2.size();

        priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(compare)> pq(compare);

        pq.push({0, 0});
        vector<vector<int>> answer;

        while (k > 0 && !pq.empty())
        {
            pair<int, int> p = pq.top();
            pq.pop();

            int num1_index = p.first;
            int num2_index = p.second;

            answer.push_back({nums1[num1_index], nums2[num2_index]});
            k--;

            if (num1_index == 0 && num2_index + 1 < nums2_size)
                pq.push({num1_index, num2_index + 1});

            if (num1_index + 1 < nums1_size)
                pq.push({num1_index + 1, num2_index});
        }
        return answer;
    }
};