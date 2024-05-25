class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        vector<int> mpp(256, -1);
        int len = 0;
        int left = 0, right = 0;
        int n = s.size();

        while (right < n) {
            
            if (mpp[s[right]] != -1) left = max(left, mpp[s[right]]+1);

            mpp[s[right]] = right;

            if (right - left +  1 > len) len = right - left + 1;       
            right++;     
        }

        return len;
    }
};