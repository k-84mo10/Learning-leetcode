class Solution
{
public:
    int maxProfit(vector<int> &prices)
    {
        int cost_price = 0;
        int profit = 0;
        int prices_num = prices.size();
        bool have_stock = false;

        if (prices_num == 1)
            return 0;

        if (prices[0] <= prices[1])
        {
            cost_price = prices[0];
            have_stock = true;
        }

        for (int i = 1; i < prices_num - 1; i++)
        {
            if (have_stock && prices[i] > prices[i + 1])
            {
                profit += prices[i] - cost_price;
                cost_price = 0;
                have_stock = false;
            }
            if (!have_stock && prices[i] <= prices[i - 1] && prices[i] < prices[i + 1])
            {
                cost_price = prices[i];
                have_stock = true;
            }
        }

        if (have_stock && prices[prices_num - 1] > cost_price)
            profit += prices[prices_num - 1] - cost_price;

        return profit;
    }
};