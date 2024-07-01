class Solution {
public:
    int firstUniqChar(string s) {
        vector<int> countCharacter(256, 0);
        int n = s.size();
        for (int i = 0; i < n; i++) {
            countCharacter[s[i]] += 1;
        }
        for (int i = 0; i < n; i++) {
            if (countCharacter[s[i]] == 1) return i;
        }
        return -1;
    }
};