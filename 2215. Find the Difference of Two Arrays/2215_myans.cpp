class Solution {
public:
    vector<vector<int>> findDifference(vector<int>& nums1, vector<int>& nums2) {
        std::sort(nums1.begin(), nums1.end());
        std::sort(nums2.begin(), nums2.end());

        std::vector<int> answer1;
        std::vector<int> answer2;

        int i = 0;
        int j = 0;

        while (i < nums1.size() && j < nums2.size()) {
            while (i < nums1.size()-1 && nums1[i] == nums1[i+1]) i++;
            while (j < nums2.size()-1 && nums2[j] == nums2[j+1]) j++;
            if (nums1[i] < nums2[j]) {
                answer1.push_back(nums1[i]);
                i++;
            } else if (nums1[i] > nums2[j]) {
                answer2.push_back(nums2[j]);
                j++;
            } else {
                i++;
                j++;
            }
        }

        while (i < nums1.size()) {
            while (i < nums1.size()-1 && nums1[i] == nums1[i+1]) i++;
            answer1.push_back(nums1[i]);
            i++;
        }
        while (j < nums2.size()) {
            while (j < nums2.size()-1 && nums2[j] == nums2[j+1]) j++;
            answer2.push_back(nums2[j]);
            j++;
        }

        return std::vector{answer1, answer2};
    }
};