class Solution
{
public:
    string removeStars(string s)
    {
        int left = 0;
        int right = 0;

        while (right < s.size())
        {
            if (s[right] == '*')
            {
                left--;
            }
            else
            {
                if (left != right)
                {
                    s.erase(left, right - left);
                    right = left;
                }
                left++;
            }
            right++;
        }

        if (left != right)
        {
            s.erase(left, right - left);
            right = left;
        }

        return s;
    }
};