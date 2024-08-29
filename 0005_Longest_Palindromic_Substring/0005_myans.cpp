class Solution
{
public:
    string longestPalindrome(string s)
    {
        int s_size = s.size();
        if (s_size == 1)
            return s;

        int max_len = 1;
        int max_len_start = 0;

        for (int i = 0; i < s_size; i++)
        {
            int left = i;
            int right = i;

            while (right < s_size - 1 && s[right + 1] == s[i])
            {
                right += 1;
            }

            while (left >= 1 && right < s_size - 1 && s[left - 1] == s[right + 1])
            {
                left -= 1;
                right += 1;
            }

            if (max_len < right - left + 1)
            {
                max_len = right - left + 1;
                max_len_start = left;
            }
        }

        string answer(s, max_len_start, max_len);
        return answer;
    }
};