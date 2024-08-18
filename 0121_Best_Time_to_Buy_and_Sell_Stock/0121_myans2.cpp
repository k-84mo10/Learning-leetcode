class Solution
{
public:
    int maxProfit(vector<int> &prices)
    {
        int min_price = prices[0];
        int price_num = prices.size();
        int max_profit = 0;
        for (int i = 0; i < price_num; i++)
        {
            if (min_price > prices[i])
            {
                min_price = prices[i];
            }
            else
            {
                max_profit = max(max_profit, prices[i] - min_price);
            }
        }
        return max_profit;
    }
};