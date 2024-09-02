class Solution
{
public:
    int chalkReplacer(vector<int> &chalk, int k)
    {
        vector<long> prefix_sum(chalk.size());
        prefix_sum[0] = chalk[0];
        for (int i = 1; i < chalk.size(); i++)
        {
            prefix_sum[i] = prefix_sum[i - 1] + chalk[i];
        }
        k = k % prefix_sum[chalk.size() - 1];

        int left = 0;
        int right = chalk.size() - 1;
        while (left <= right)
        {
            int medium = left + (right - left) / 2;
            if (prefix_sum[medium] == k)
            {
                return medium + 1;
            }
            else if (prefix_sum[medium] > k)
            {
                right = medium - 1;
            }
            else
            {
                left = medium + 1;
            }
        }
        return left;
    }
};