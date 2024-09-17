class Solution
{
public:
    string reverseWords(string s)
    {
        vector<string> words;
        string current = "";
        s = s + " ";

        for (char &word : s)
        {
            if (word == ' ')
            {
                if (current == "")
                    continue;
                words.push_back(current);
                current = "";
            }
            else
            {
                current.push_back(word);
            }
        }

        reverse(words.begin(), words.end());

        string answer = "";
        for (string &str : words)
        {
            answer = answer + str + " ";
        }
        answer.pop_back();

        return answer;
    }
};