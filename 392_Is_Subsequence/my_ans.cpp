class Solution {
public:
    bool isSubsequence(string s, string t) {
        int ptr = 0;
        if (t.size() == 0 && s.size() != 0) return false;
        for (int i = 0; i < s.size(); i++) {
            if (ptr == t.size()) return false;
            for (int j = ptr; j < t.size(); j++) {
                if (s[i] == t[j]) {
                    ptr = j + 1;
                    break;
                }
                if (j == t.size() - 1) return false;
            }
        } 
        return true;
    }
};