class Solution {
public:
    int getCommon(vector<int>& nums1, vector<int>& nums2) {
        int idx1 = 0; 
        int idx2 = 0;

        int len1 = nums1.size();
        int len2 = nums2.size();

        while (idx1 < len1 && idx2 < len2) {
            int n1 = nums1[idx1];
            int n2 = nums2[idx2];

            if (n1 == n2) {
                return n1;
            } else if (n1 < n2) {
                idx1 += 1;
            } else {
                idx2 += 1;
            }
        }

        return -1;
    }
};