class Solution {
public:
    int firstUniqChar(string s) {
        vector<int> mpp(256, -1);
        int n = s.size();
        for (int i = 0; i < n; i++) {
            if (mpp[s[i]] == -1) {
                mpp[s[i]] = i;
            } else {
                mpp[s[i]] = -2;
            }
        }
        int first = pow(10, 5)+1;
        for (int i = 0; i < mpp.size(); i++) {
            if (mpp[i] != -1 && mpp[i] != -2) {
                if (first > mpp[i]) first = mpp[i];
            }
        }
        if (first == pow(10, 5)+1) first = -1;
        return first;
    }
};