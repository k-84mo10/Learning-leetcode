class Solution
{
public:
    int coinChange(vector<int> &coins, int amount)
    {
        vector<int> can_represent_amount(amount + 1, 10001);
        int coins_size = coins.size();

        can_represent_amount[0] = 0;

        for (int i = 0; i <= amount; i++)
        {
            for (int coin : coins)
            {
                if (i < coin)
                    continue;
                can_represent_amount[i] = min(can_represent_amount[i - coin] + 1, can_represent_amount[i]);
            }
        }

        return (can_represent_amount[amount] != 10001) ? can_represent_amount[amount] : -1;
    }
};