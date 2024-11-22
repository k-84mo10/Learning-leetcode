class Solution {
public:
    vector<vector<int>> findDifference(vector<int>& nums1, vector<int>& nums2) {
        std::set<int> s1;
        std::set<int> s2;

        for (int& num : nums1) {
            s1.insert(num);
        }

        for (int& num : nums2) {
            s2.insert(num);
        }

        vector<int> answer1;
        vector<int> answer2;
        for (auto s : s1) {
            if (s2.find(s) == s2.end()) answer1.push_back(s);
        }
        for (auto s : s2) {
            if (s1.find(s) == s1.end()) answer2.push_back(s);
        }

        return std::vector{answer1, answer2};
    }
};