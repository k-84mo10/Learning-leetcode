class Solution
{
public:
    int reverse(int x)
    {
        int y = 0;
        while (1)
        {
            y += x % 10;
            x = x / 10;
            if (x == 0)
                break;
            if (y > numeric_limits<int>::max() / 10 || y < numeric_limits<int>::min() / 10)
                return 0;
            y *= 10;
        }

        return y;
    }
};