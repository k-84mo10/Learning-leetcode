class Solution
{
public:
    bool isDayEnough(vector<int> &weights, int days, int capasity)
    {
        int need_days = 1;
        int capasity_left = capasity;
        for (int &weight : weights)
        {
            if (weight <= capasity_left)
            {
                capasity_left -= weight;
            }
            else
            {
                need_days++;
                if (need_days > days || weight > capasity)
                    return false;
                capasity_left = capasity - weight;
            }
        }
        return true;
    }

    int shipWithinDays(vector<int> &weights, int days)
    {
        int capasity_low = 0;
        int capasity_high = accumulate(weights.begin(), weights.end(), 0);

        int capasity_minimum = capasity_high;

        while (capasity_low <= capasity_high)
        {
            int capasity_mid = capasity_low + (capasity_high - capasity_low) / 2;
            if (isDayEnough(weights, days, capasity_mid))
            {
                capasity_minimum = capasity_mid;
                capasity_high = capasity_mid - 1;
            }
            else
            {
                capasity_low = capasity_mid + 1;
            }
        }

        return capasity_minimum;
    }
};