class Solution
{
public:
    int compress(vector<char> &chars)
    {
        int i = 0;
        int group_num = 0;
        while (i < chars.size())
        {
            if (i != 0 && chars[i] == chars[i - 1])
            {
                group_num++;
                chars.erase(chars.begin() + i);
                continue;
            }
            if (group_num >= 2)
            {
                int start = i;
                while (group_num > 0)
                {
                    char c = (group_num % 10) + '0';
                    chars.insert(chars.begin() + i, c);
                    i++;
                    group_num /= 10;
                }
                reverse(chars.begin() + start, chars.begin() + i);
            }
            group_num = 1;
            i++;
        }

        if (group_num >= 2)
        {
            int start = i;
            while (group_num > 0)
            {
                char c = (group_num % 10) + '0';
                chars.push_back(c);
                group_num /= 10;
                i++;
            }
            reverse(chars.begin() + start, chars.end());
        }

        return i;
    }
};