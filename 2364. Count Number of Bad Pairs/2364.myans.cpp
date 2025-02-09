class Solution {
    public:
        long long countBadPairs(vector<int>& nums) {
            const int nums_size = nums.size();
            std::unordered_map<int,int> differences;
    
            for (int i = 0; i < nums.size(); i++) {
                const int diff = nums[i] - i;
                differences[diff] += 1;
            }
    
            long long good_pair_num = 0;
            for (auto& pair : differences) {
                good_pair_num += (static_cast<long long>(pair.second) * (pair.second - 1)) / 2LL;
            }
    
            const long long all_pair = (static_cast<long long>(nums_size) * (nums_size - 1)) / 2LL;
            return all_pair - good_pair_num;
        }
    };