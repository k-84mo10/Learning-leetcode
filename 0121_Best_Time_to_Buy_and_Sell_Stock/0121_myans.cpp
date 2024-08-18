class Solution
{
public:
    int maxProfit(vector<int> &prices)
    {
        int prices_num = prices.size();

        int max_profit = 0;
        int current_max_price = 0;
        int current_min_price = prices[0];

        for (int i = 1; i < prices_num; i++)
        {
            if (prices[i] < current_min_price)
            {
                current_min_price = prices[i];
                current_max_price = 0;
            }
            if (prices[i] > current_max_price)
            {
                current_max_price = prices[i];
                max_profit = max(max_profit, current_max_price - current_min_price);
            }
        }
        return max_profit;
    }
};