class Solution
{
public:
    string reverseVowels(string s)
    {
        stack<char> vowl;
        queue<int> vowl_position;
        for (int i = 0; i < s.size(); i++)
        {
            if (s[i] == 'a' || s[i] == 'e' || s[i] == 'i' || s[i] == 'o' || s[i] == 'u' || s[i] == 'A' || s[i] == 'E' || s[i] == 'I' || s[i] == 'O' || s[i] == 'U')
            {
                vowl.push(s[i]);
                vowl_position.push(i);
            }
        }
        while (!vowl_position.empty())
        {
            int num = vowl_position.front();
            vowl_position.pop();
            s[num] = vowl.top();
            vowl.pop();
        }

        return s;
    }
};