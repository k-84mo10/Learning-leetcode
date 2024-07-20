class Solution
{
public:
    double myPow(double x, int n)
    {
        long exponent = n;
        if (exponent < 0)
            exponent = -1 * exponent;
        double base = (n > 0) ? x : 1 / x;
        double ans = 1;

        while (exponent)
        {
            if (exponent & 1)
            {
                ans = ans * base;
            }
            base = base * base;
            exponent = exponent >> 1;
        }
        return ans;
    }
};