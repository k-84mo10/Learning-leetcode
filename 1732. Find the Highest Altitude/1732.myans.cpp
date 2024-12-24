class Solution {
public:
    int largestAltitude(vector<int>& gain) {
        int highest = 0;
        int current = 0;
        for (int& num : gain) {
            current += num;
            highest = max(current, highest);
        }
        return highest;
    }
};