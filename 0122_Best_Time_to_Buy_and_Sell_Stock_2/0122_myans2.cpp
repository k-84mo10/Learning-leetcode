class Solution
{
public:
    int maxProfit(vector<int> &prices)
    {
        int cost_price = prices[0];
        int profit = 0;

        for (int i = 0; i < prices.size(); i++)
        {
            if (prices[i] > cost_price)
            {
                profit += prices[i] - cost_price;
            }
            cost_price = prices[i];
        }
        return profit;
    }
};