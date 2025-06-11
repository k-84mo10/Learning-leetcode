class Solution {
public:
    int strStr(string haystack, string needle) {
        size_t position = haystack.find(needle);
        return (position != string::npos) ? position : -1;
    }
};