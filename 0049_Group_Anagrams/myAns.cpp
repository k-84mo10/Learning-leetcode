class Solution
{
public:
    vector<vector<string>> groupAnagrams(vector<string> &strs)
    {
        unordered_map<string, int> anagram_address;
        vector<vector<string>> anagram;
        int index = 0;
        for (string str : strs)
        {
            string sorted_str = str;
            sort(sorted_str.begin(), sorted_str.end());
            if (anagram_address.find(sorted_str) != anagram_address.end())
            {
                int address = anagram_address[sorted_str];
                anagram[address].push_back(str);
            }
            else
            {
                anagram_address[sorted_str] = index;
                vector<string> new_anagram(1, str);
                anagram.push_back(new_anagram);
                index++;
            }
        }
        return anagram;
    }
};