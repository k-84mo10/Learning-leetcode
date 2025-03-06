class Solution {
    public:
        vector<int> findMissingAndRepeatedValues(vector<vector<int>>& grid) {
            int n = grid.size();
            std::vector<int> count(n * n + 1, 0);
            int repeated = -1, missing = -1;
    
            for (auto& row : grid) {
                for (int num : row) {
                    if (++count[num] == 2) repeated = num;
                }
            }
    
            missing = std::find(count.begin() + 1, count.end(), 0) - count.begin();
            return {repeated, missing};
        }
    };
    