class Solution {
public:
    int minMoves(vector<int>& nums, int limit) {
        vector<int> diff(2*limit+2);
        int n = nums.size();

        for (int i = 0; i < (n+1)/2; i++) {
            int a = min(nums[i], nums[n-1-i]);
            int b = max(nums[i], nums[n-1-i]);

            diff[2] += 2;
            diff[a+1] -= 1;
            diff[a+b] -= 1;
            diff[a+b+1] += 1;
            diff[b+limit+1] += 1;
        }

        int min_ops = INT_MAX;
        int current_ops = 0;
        for (int i = 2; i < 2*limit+1; i++) {
            current_ops += diff[i];
            min_ops = min(min_ops, current_ops);
        }

        return min_ops;
    }
};