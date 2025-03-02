class Solution {
public:
    vector<vector<int>> mergeArrays(vector<vector<int>>& nums1, vector<vector<int>>& nums2) {
        std::vector<std::vector<int>> merged_array;
        int i = 0, j = 0;

        while (i < nums1.size() && j < nums2.size()) {
            if (nums1[i][0] == nums2[j][0]) {
                merged_array.push_back({nums1[i][0], nums1[i++][1]+nums2[j++][1]});
            } else if (nums1[i][0] < nums2[j][0]) {
                merged_array.push_back(nums1[i++]);
            } else {
                merged_array.push_back(nums2[j++]);
            }
        }

        while (i < nums1.size()) merged_array.push_back(nums1[i++]);
        while (j < nums2.size()) merged_array.push_back(nums2[j++]);

        return merged_array;
    }
};
