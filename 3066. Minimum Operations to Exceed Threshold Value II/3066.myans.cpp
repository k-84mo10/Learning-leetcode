class Solution {
    public:
        int minOperations(vector<int>& nums, int k) {
            std::priority_queue<long, std::vector<long>, std::greater<long>> pq;
    
            for (int num : nums) {
                pq.push(num);
            }
    
            int count = 0;
            while(pq.top() < k) {
                long first = pq.top();
                pq.pop();
                long second = pq.top();
                pq.pop();
                pq.push(first*2+second);
                count ++;
            }
    
            return count;
        }
    };