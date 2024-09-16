class Solution
{
public:
    string mergeAlternately(string word1, string word2)
    {
        int minLength = min(word1.size(), word2.size());
        string merged = "";
        for (int i = 0; i < minLength; i++)
        {
            merged.push_back(word1[i]);
            merged.push_back(word2[i]);
        }

        if (word1.size() > minLength)
        {
            for (int i = minLength; i < word1.size(); i++)
            {
                merged.push_back(word1[i]);
            }
        }

        if (word2.size() > minLength)
        {
            for (int i = minLength; i < word2.size(); i++)
            {
                merged.push_back(word2[i]);
            }
        }

        return merged;
    }
};