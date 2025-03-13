class Solution {
    public:
        int minZeroArray(vector<int>& nums, vector<vector<int>>& queries) {
            int zero_num = 0;   // the number of zero in nums
    
            // count the number of zero in nums
            for (int i = 0; i < nums.size(); i++) {
                if (nums[i] == 0) zero_num ++;
            }
            if (zero_num == nums.size()) return 0;
    
            // 
            for (int i = 0; i < queries.size(); i++) {
                for (int j = queries[i][0]; j <= queries[i][1]; j++) {
                    if (nums[j] == 0) continue;
                    nums[j] -= queries[i][2];
                    if (nums[j] <= 0) {
                        zero_num ++;
                        nums[j] = 0;
                    }
                }
                if (zero_num == nums.size()) return i+1;
            }
            return -1;
        }
    };