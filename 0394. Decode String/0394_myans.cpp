class Solution
{
public:
    string decodeString(string s)
    {
        string answer;
        int right = 0;
        while (right < s.size())
        {
            if (isdigit(s[right]))
            {
                int num = 0;
                while (isdigit(s[right]))
                {
                    num = num * 10 + (s[right] - '0');
                    right++;
                }
                right++;

                int left = right;
                int leftbra = 1, rightbra = 0;

                while (leftbra > rightbra && right < s.size())
                {
                    if (s[right] == '[')
                        leftbra++;
                    if (s[right] == ']')
                        rightbra++;
                    right++;
                }

                string sub = s.substr(left, right - left - 1);
                string decoded_sub = decodeString(sub);

                for (int i = 0; i < num; i++)
                {
                    answer += decoded_sub;
                }
            }
            else
            {
                answer.push_back(s[right]);
                right++;
            }
        }
        return answer;
    }
};