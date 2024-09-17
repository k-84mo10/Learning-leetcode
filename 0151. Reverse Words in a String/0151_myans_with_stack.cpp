class Solution
{
public:
    string reverseWords(string s)
    {
        int s_size = s.size();
        int index = 0;
        stack<string> words;

        while (index < s_size)
        {
            int start = index;
            int count = 0;
            while (s[index] != ' ' && index < s_size)
            {
                index++;
                count++;
            }
            if (count != 0)
                words.push(s.substr(start, count));
            index++;
        }

        string answer = "";
        while (!words.empty())
        {
            answer += words.top() + " ";
            words.pop();
        }
        answer.pop_back();

        return answer;
    }
};