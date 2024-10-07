class Solution
{
public:
    int maxVowels(string s, int k)
    {
        vector<int> cumulative_sum(s.size(), 0);
        if (s[0] == 'a' || s[0] == 'e' || s[0] == 'i' || s[0] == 'o' || s[0] == 'u')
        {
            cumulative_sum[0] = 1;
        }

        for (int i = 1; i < s.size(); i++)
        {
            if (s[i] == 'a' || s[i] == 'e' || s[i] == 'i' || s[i] == 'o' || s[i] == 'u')
            {
                cumulative_sum[i] = cumulative_sum[i - 1] + 1;
            }
            else
            {
                cumulative_sum[i] = cumulative_sum[i - 1];
            }
        }

        int max_vowels = cumulative_sum[k - 1];
        for (int i = 0; i < s.size() - k; i++)
        {
            max_vowels = max(max_vowels, cumulative_sum[i + k] - cumulative_sum[i]);
        }

        return max_vowels;
    }
};