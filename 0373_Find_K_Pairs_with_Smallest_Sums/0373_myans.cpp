class Solution
{
public:
    vector<vector<int>> kSmallestPairs(vector<int> &nums1, vector<int> &nums2, int k)
    {
        auto compare = [](vector<int> a, vector<int> b)
        {
            return a[0] + a[1] > b[0] + b[1];
        };

        priority_queue<vector<int>, vector<vector<int>>, decltype(compare)> q;

        for (int &num1 : nums1)
        {
            for (int &num2 : nums2)
            {
                vector<int> newpair{num1, num2};
                q.push(newpair);
            }
        }

        vector<vector<int>> answer;
        for (int i = 0; i < k; i++)
        {
            vector<int> smallpair = q.top();
            q.pop();

            answer.push_back(smallpair);
        }

        return answer;
    }
};