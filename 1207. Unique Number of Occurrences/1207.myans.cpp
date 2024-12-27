class Solution {
public:
    bool uniqueOccurrences(vector<int>& arr) {
        std::set<int> s;

        sort(arr.begin(), arr.end());

        int i = 0;
        while (i < arr.size()) {
            int current = arr[i];
            int count = 0;
            while (i < arr.size() && arr[i] == current) {
                count ++;
                i ++;
            }
            if (s.find(count) != s.end()) {
                return false;
            } else {
                s.insert(count);
            }
        }
        return true;
    }
};