class Solution
{
public:
    double myPow(double x, int n)
    {
        if (n == 0)
        {
            return 1;
        }
        if (n == 1)
        {
            return x;
        }
        if (n == numeric_limits<int>::lowest())
        {
            return myPow(x, n + 1) * (1 / x);
        }
        if (n < 0)
        {
            x = 1 / x;
            n = n * -1;
        }

        double base = myPow(x, n / 2);
        double square = base * base;
        if (n % 2 == 1)
            square = square * x;

        return square;
    }
};