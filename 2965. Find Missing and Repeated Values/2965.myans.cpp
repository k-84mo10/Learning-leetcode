class Solution {
    public:
        vector<int> findMissingAndRepeatedValues(vector<vector<int>>& grid) {
            int n = grid.size();
            std::vector<bool> is_once(n*n+1, false);
    
            std::vector<int> answer;
    
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < n; j++) {
                    if (is_once[grid[i][j]]) answer.push_back(grid[i][j]);
                    is_once[grid[i][j]] = true;
                }
            } 
    
            for (int i = 1; i <= n*n; i++) {
                if (!is_once[i]) answer.push_back(i);
            }
            
            return answer;
        }
    };