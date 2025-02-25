class Solution {
public:
    int numOfSubarrays(vector<int>& arr) {
        const int MOD = 1e9+7;
        const int arr_size = arr.size();

        std::vector<int> dp_odd(arr_size);
        std::vector<int> dp_even(arr_size);

        dp_odd[0] = (arr[0] % 2 == 1) ? 1 : 0;
        dp_even[0] = (arr[0] % 2 == 1) ? 0 : 1;

        for (int i = 1; i < arr_size; i++) {
            if (arr[i] % 2 == 1) {
                dp_odd[i] = (dp_even[i-1] + 1) % MOD;
                dp_even[i] = dp_odd[i-1];
            } else {
                dp_odd[i] = dp_odd[i-1];
                dp_even[i] = (dp_even[i-1] + 1) % MOD;
            }
        }

        int sum = 0;
        for (auto odd : dp_odd) {
            sum += odd;
            sum %= MOD;
        }
        
        return sum;
    }
};
