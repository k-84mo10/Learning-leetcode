class Solution
{
public:
    vector<int> successfulPairs(vector<int> &spells, vector<int> &potions, long long success)
    {
        sort(potions.begin(), potions.end());
        int potions_size = potions.size();

        vector<int> output;

        for (int &spell : spells)
        {
            int left = 0;
            int right = potions_size - 1;
            while (left <= right)
            {
                int mid = (left + right) / 2;
                long long midValue = static_cast<long long>(potions[mid]) * spell;
                if (midValue >= success)
                {
                    right = mid - 1;
                }
                else
                {
                    left = mid + 1;
                }
            }
            output.push_back(potions_size - left);
        }

        return output;
    }
};