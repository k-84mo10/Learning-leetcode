class Solution {
public:
    bool repeatedSubstringPattern(string s) {
        std::string doubled = s + s;
        std::string trimed = doubled.substr(1, doubled.size() - 2);

        return trimed.find(s) != string::npos;
    }
};