class Solution {
    public:
        int minZeroArray(vector<int>& nums, vector<vector<int>>& queries) {
            int left = 0, right = queries.size();
            if (!canFormZeroArray(nums, queries, right)) return -1;
    
            while (left <= right) {
                int middle = (left + right) / 2;
                if (canFormZeroArray(nums, queries, middle)) {
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            }
            return left;
        }
    
        bool canFormZeroArray(vector<int>& nums, vector<vector<int>>& queries, int query_num) {
            vector<int> difference(nums.size()+1, 0);
            for (int i = 0; i < query_num; i++) {
                int start = queries[i][0], end = queries[i][1];
                int val = queries[i][2];
    
                difference[start] += val;
                difference[end+1] -= val; 
            }
    
            int sum = 0;
            for (int i = 0; i < nums.size(); i++) {
                sum += difference[i];
                if (sum < nums[i]) return false;
            }
            return true;
        }
    };