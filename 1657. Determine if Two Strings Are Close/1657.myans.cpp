class Solution
{
public:
    bool closeStrings(string word1, string word2)
    {
        vector<bool> is_alphabet_used1(26, false);
        vector<bool> is_alphabet_used2(26, false);
        vector<int> alphabet_num1(26, 0);
        vector<int> alphabet_num2(26, 0);

        for (char &word : word1)
        {
            int num = word - 'a';
            if (!is_alphabet_used1[num])
                is_alphabet_used1[num] = true;
            alphabet_num1[num]++;
        }

        for (char &word : word2)
        {
            int num = word - 'a';
            if (!is_alphabet_used2[num])
                is_alphabet_used2[num] = true;
            alphabet_num2[num]++;
        }

        sort(alphabet_num1.begin(), alphabet_num1.end());
        sort(alphabet_num2.begin(), alphabet_num2.end());

        for (int i = 0; i < 26; i++)
        {
            if (is_alphabet_used1[i] != is_alphabet_used2[i] || alphabet_num1[i] != alphabet_num2[i])
                return false;
        }

        return true;
    }
};