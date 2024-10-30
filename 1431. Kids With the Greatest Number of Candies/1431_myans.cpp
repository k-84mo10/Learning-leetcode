class Solution
{
public:
    vector<bool> kidsWithCandies(vector<int> &candies, int extraCandies)
    {
        int max_candies = 0;
        for (int &candy : candies)
        {
            max_candies = max(max_candies, candy);
        }

        vector<bool> is_maximum(candies.size(), false);
        for (int i = 0; i < candies.size(); i++)
        {
            is_maximum[i] = (candies[i] + extraCandies >= max_candies) ? true : false;
        }

        return is_maximum;
    }
};