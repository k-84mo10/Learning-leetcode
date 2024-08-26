class Solution
{
public:
    int ladderLength(string beginWord, string endWord, vector<string> &wordList)
    {
        unordered_map<string, int> wordmap;

        for (string &str : wordList)
        {
            wordmap[str] = 10000;
        }

        queue<string> q;
        q.push(beginWord);
        wordmap[beginWord] = 1;
        int wordlength = beginWord.size();

        while (!q.empty())
        {
            string str = q.front();
            q.pop();
            int dict = wordmap[str];

            for (int i = 0; i < wordlength; i++)
            {
                string ladderword = str;
                for (char c = 'a'; c <= 'z'; c++)
                {
                    ladderword[i] = c;
                    if (wordmap.find(ladderword) != wordmap.end())
                    {
                        if (ladderword == endWord)
                            return dict + 1;
                        if (dict + 1 < wordmap[ladderword])
                        {
                            q.push(ladderword);
                            wordmap[ladderword] = dict + 1;
                        }
                    }
                }
            }
        }
        return 0;
    }
};