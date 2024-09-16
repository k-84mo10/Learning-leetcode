class Solution
{
public:
    bool canPlaceFlowers(vector<int> &flowerbed, int n)
    {
        int enable_space_num = 0;
        int flower_num = flowerbed[0];

        for (int i = 1; i <= flowerbed.size(); i++)
        {
            if (i < flowerbed.size())
                flower_num += flowerbed[i];
            if (flower_num == 0)
            {
                enable_space_num += 1;
                flowerbed[i - 1] = 1;
                flower_num += 1;
            }
            if (i >= 2)
                flower_num -= flowerbed[i - 2];
            if (enable_space_num >= n)
                return true;
        }

        return false;
    }
};